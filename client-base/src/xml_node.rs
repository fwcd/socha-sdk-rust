use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;
use std::io::{Read, Write};
use xml::reader::{EventReader, XmlEvent as XmlReadEvent};
use xml::writer::{EventWriter, XmlEvent as XmlWriteEvent};
use log::{warn, error};
use crate::util::SCResult;
use crate::error::SCError;

/// A deserialized, in-memory tree-representation
/// of an XML node.
#[derive(Debug, Default)]
pub struct XmlNode {
	name: String,
	data: String,
	attributes: HashMap<String, String>,
	childs: Vec<XmlNode>
}

/// A builder that makes the construction of new
/// XML nodes more convenient.
pub struct XmlNodeBuilder<'a> {
	name: &'a str,
	data: &'a str,
	attributes: HashMap<String, String>,
	childs: Vec<XmlNode>
}

/// Indicates that the type can be created from an XML node.
pub trait FromXmlNode where Self: Sized {
	fn from_node(node: &XmlNode) -> SCResult<Self>;
}

impl XmlNode {
	/// Creates a new XML node builder.
	pub fn new(name: &str) -> XmlNodeBuilder {
		XmlNodeBuilder::new(name)
	}

	/// Deserializes an XML node tree
	/// from the given XML event reader.
	pub fn read_from<R>(reader: &mut EventReader<R>) -> SCResult<XmlNode> where R: Read {
		let mut node_stack = VecDeque::<XmlNode>::new();
		
		loop {
			match reader.next() {
				Ok(XmlReadEvent::StartElement { name, attributes, .. }) => {
					let node = XmlNode {
						name: name.local_name,
						data: String::new(),
						attributes: attributes.iter().cloned().map(|attr| (attr.name.local_name, attr.value)).collect(),
						childs: Vec::new()
					};
					node_stack.push_back(node);
				},
				Ok(XmlReadEvent::EndElement { name }) => {
					if let Some(node) = node_stack.pop_back() {
						if let Some(mut parent) = node_stack.pop_back() {
							parent.childs.push(node);
							node_stack.push_back(parent);
						} else {
							return Ok(node);
						}
					} else {
						error!("Found closing element </{}> without an opening element before", name);
					}
				},
				Ok(XmlReadEvent::Characters(content)) => {
					if let Some(node) = node_stack.back_mut() {
						node.data += content.as_str();
					} else {
						warn!("Found characters {} outside of any node", content);
					}
				},
				Err(e) => return Err(e.into()),
				_ => ()
			}
		}
	}
	
	/// Serializes the node to an XML string using a tree traversal.
	pub fn write_to<W>(&self, writer: &mut EventWriter<W>) -> SCResult<()> where W: Write {
		let mut start_element = XmlWriteEvent::start_element(self.name.as_str());
		for (key, value) in &self.attributes {
			start_element = start_element.attr(key.as_str(), value.as_str());
		}
		writer.write(start_element)?;
		
		if !self.data.is_empty() {
			writer.write(XmlWriteEvent::characters(&self.data))?;
		}

		for child in &self.childs {
			child.write_to(writer)?;
		}
		
		writer.write(XmlWriteEvent::end_element())?;
		Ok(())
	}
	
	/// Fetches the node's tag name.
	pub fn name(&self) -> &str {
		self.name.as_str()
	}
	
	/// Fetches the node's textual contents.
	pub fn data(&self) -> &str {
		self.data.as_str()
	}
	
	/// Fetches an attribute's value by key.
	pub fn attribute(&self, key: &str) -> SCResult<&str> {
		self.attributes.get(key).map(|s| s.as_str()).ok_or_else(|| format!("No attribute with key '{}' found in <{}>!", key, self.name).into())
	}
	
	/// Finds the first child element with the provided tag name.
	pub fn child_by_name<'a, 'n: 'a>(&'a self, name: &'n str) -> SCResult<&'a XmlNode> {
		self.childs_by_name(name).next().ok_or_else(|| format!("No <{}> found in <{}>!", name, self.name).into())
	}
	
	/// Fetches a list of all child elements matching the provided tag name.
	pub fn childs_by_name<'a, 'n: 'a>(&'a self, name: &'n str) -> impl Iterator<Item=&'a XmlNode> + 'a {
		self.childs.iter().filter(move |c| c.name == name)
	}
}


impl<'a> XmlNodeBuilder<'a> {
	/// Creates a new XML node builder with the
	/// specified tag name.
	pub fn new(name: &'a str) -> Self {
		Self { name: name, data: "", attributes: HashMap::new(), childs: Vec::new() }
	}
	
	/// Sets the tag name of the XML node.
	pub fn name(mut self, name: &'a str) -> Self {
		self.name = name;
		self
	}
	
	/// Sets the contents of the XML node.
	pub fn data(mut self, data: &'a str) -> Self {
		self.data = data;
		self
	}
	
	/// Adds the specified attributes.
	pub fn attributes(mut self, attributes: impl IntoIterator<Item=(String, String)>) -> Self {
		self.attributes.extend(attributes);
		self
	}
	
	/// Adds the specified attribute.
	pub fn attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
		self.attributes.insert(key.into(), value.into());
		self
	}
	
	/// Adds the specified children.
	pub fn childs(mut self, childs: impl IntoIterator<Item=XmlNode>) -> Self {
		self.childs.extend(childs);
		self
	}
	
	/// Adds the specified child.
	pub fn child(mut self, child: impl Into<XmlNode>) -> Self {
		self.childs.push(child.into());
		self
	}
	
	/// Tries adding the specified child.
	pub fn try_child(mut self, child: impl TryInto<XmlNode, Error=SCError>) -> SCResult<Self> {
		self.childs.push(child.try_into()?);
		Ok(self)
	}
	
	/// Builds the XML node.
	pub fn build(self) -> XmlNode {
		XmlNode {
			name: self.name.to_owned(),
			data: self.data.to_owned(),
			attributes: self.attributes,
			childs: self.childs
		}
	}
}

impl<'a> Default for XmlNodeBuilder<'a> {
	fn default() -> Self {
		Self::new("")
	}
}

impl<'a> From<XmlNodeBuilder<'a>> for XmlNode {
	fn from(builder: XmlNodeBuilder<'a>) -> Self { builder.build() }
}
