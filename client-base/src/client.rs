use serde::{Serialize, Deserialize};
use std::{fmt, net::TcpStream, str::FromStr};
use std::io::{self, BufWriter, BufReader, BufRead, Write};
use log::{info, debug, warn, error};
use quick_xml::{Reader as XmlReader, events::Event as XmlEvent};
use quick_xml::se::to_writer;
use quick_xml::de::from_reader;
use crate::util::SCResult;
use crate::plugin::{SCPlugin, HasPlayerColor, HasTurn};
use crate::protocol::{Packet, Joined, Left, Room, Event, GameResult};

/// A handler that implements the game player's
/// behavior, usually employing some custom move
/// selection strategy.
pub trait SCClientDelegate {
    /// The plugin defining which types are
    /// representing various parts of the game.
    type Plugin: SCPlugin;

    /// Invoked whenever the game state updates.
    fn on_update_state(&mut self, _state: &<Self::Plugin as SCPlugin>::GameState) {}
    
    /// Invoked when the game ends.
    fn on_game_end(&mut self, _result: GameResult<<Self::Plugin as SCPlugin>::Player>) {}
    
    /// Invoked when the welcome message is received
    /// with the player's color.
    fn on_welcome_message(&mut self, _color: &<Self::Plugin as SCPlugin>::PlayerColor) {}
    
    /// Requests a move from the delegate. This method
    /// should implement the "main" game logic.
    fn request_move(&mut self, state: &<Self::Plugin as SCPlugin>::GameState, my_color: <Self::Plugin as SCPlugin>::PlayerColor) -> <Self::Plugin as SCPlugin>::Move;
}

/// A configuration that determines whether
/// the reader and/or the writer of a stream
/// should be swapped by stdio to ease debugging.
pub struct DebugMode {
    pub debug_reader: bool,
    pub debug_writer: bool
}

/// The client which handles XML requests, manages
/// the game state and invokes the delegate.
pub struct SCClient<D> where D: SCClientDelegate {
    delegate: D,
    debug_mode: DebugMode,
    game_state: Option<<D::Plugin as SCPlugin>::GameState>
}

impl<D> SCClient<D>
    where
        D: SCClientDelegate,
        <D::Plugin as SCPlugin>::PlayerColor: fmt::Display + FromStr,
        <D::Plugin as SCPlugin>::Player: Serialize + for<'de> Deserialize<'de>,
        <D::Plugin as SCPlugin>::GameState: Serialize + for<'de> Deserialize<'de>,
        <D::Plugin as SCPlugin>::Move: Serialize + for<'de> Deserialize<'de> {
    /// Creates a new client using the specified delegate.
    pub fn new(delegate: D, debug_mode: DebugMode) -> Self {
        Self { delegate: delegate, debug_mode: debug_mode, game_state: None }
    }
    
    /// Blocks the thread and begins reading XML messages
    /// from the provided address via TCP.
    pub fn run(self, host: &str, port: u16, reservation: Option<&str>) -> SCResult<()> {
        let address = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&address)?;
        info!("Connected to {}", address);
        
        {
            let mut writer = BufWriter::new(&stream);
            writer.write("<protocol>".as_bytes())?;
            
            let join_xml = match reservation {
                Some(res) => format!("<joinPrepared reservationCode=\"{}\" />", res),
                None => format!("<join gameType=\"{}\" />", D::Plugin::protocol_game_type())
            };
            info!("Sending join message {}", join_xml);
            writer.write(join_xml.as_bytes())?;
        }
        
        // Begin parsing game messages from the stream.
        // List all combinations of modes explicitly,
        // since they generate different generic instantiations
        // of `run_game`.

        let mode = &self.debug_mode;
        if mode.debug_reader && !mode.debug_writer {
            self.run_game(BufReader::new(io::stdin()), BufWriter::new(stream))?;
        } else if !mode.debug_reader && mode.debug_writer {
            self.run_game(BufReader::new(stream), BufWriter::new(io::stdout()))?;
        } else if mode.debug_reader && mode.debug_writer {
            self.run_game(BufReader::new(io::stdin()), BufWriter::new(io::stdout()))?;
        } else {
            let reader = BufReader::new(stream.try_clone()?);
            let writer = BufWriter::new(stream);
            self.run_game(reader, writer)?;
        }
        
        Ok(())
    }
    
    /// Blocks the thread and parses/handles game messages
    /// from the provided reader.
    fn run_game<R, W>(mut self, reader: R, mut writer: W) -> SCResult<()> where R: BufRead, W: Write {
        // Read initial <protocol> element

        let mut xml_reader = XmlReader::from_reader(reader);
        let mut xml_buf = Vec::new();

        info!("Waiting for initial <protocol>...");
        while !matches!(
            xml_reader.read_event(&mut xml_buf),
            Ok(XmlEvent::Start(ref e)) if e.name() == "protocol".as_bytes()
        ) {}

        let mut reader = xml_reader.into_underlying_reader();

        // Read incoming packets

        loop {
            let packet: Packet<D::Plugin> = from_reader(&mut reader).map_err(|e| format!("Could not read packet: {:?}", e))?;
            debug!("Got packet {:?}", packet);
            
            match packet {
                Packet::Room(Room { event, room_id }) => match event {
                    Event::WelcomeMessage { color } => {
                        info!("Got welcome message with color: {:?}", color);
                        self.delegate.on_welcome_message(&color);
                    },
                    Event::Memento { state } => {
                        info!("Got updated game state");
                        self.delegate.on_update_state(&state);
                        self.game_state = Some(state);
                    },
                    Event::MoveRequest => {
                        if let Some(ref state) = self.game_state {
                            let turn = state.turn();
                            let color = state.player_color();
                            info!("Got move request @ turn: {}, color: {:?}", turn, color);

                            let new_move = self.delegate.request_move(state, color);
                            let move_packet = Packet::Room(Room::<D::Plugin> {
                                room_id: room_id,
                                event: Event::Move { r#move: new_move }
                            });

                            debug!("Sending move packet {:?}", move_packet);
                            to_writer(&mut writer, &move_packet).map_err(|e| format!("Could not write packet: {:?}", e))?;
                            writer.flush()?;
                        } else {
                            error!("Got move request, which cannot be fulfilled since no game state is present!");
                        }
                    },
                    Event::Result { result } => {
                        info!("Got game result: {:?}", result);
                        self.delegate.on_game_end(result);
                    },
                    Event::Error { message } => {
                        warn!("Got error from server: {}", message);
                    },
                    _ => warn!("Could not handle event: {:?}", event)
                },
                Packet::Joined(Joined { room_id }) => info!("Joined room {}", room_id),
                Packet::Left(Left { room_id }) => info!("Left room {}", room_id),
                Packet::Close(_) => {
                    info!("Closing connection as requested by the server...");
                    break;
                }
            }
        }
        
        Ok(())
    }
}
