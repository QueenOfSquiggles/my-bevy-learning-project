use bevy::{prelude::*, utils::HashMap};
use style_description::StyleDescription;

pub mod style_description;

#[derive(Debug, Clone)]
struct UiLayoutNode {
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct UiBuilder;

#[derive(Debug, Clone)]
pub struct UiBuilderLayoutStep {
    layout: Vec<(u8, UiLayoutNode)>,
    current_indent: u8,
}

#[derive(Debug, Clone)]
pub struct UiBuilderDescriptionStep {
    description: Vec<(u8, UiLayoutNode, StyleDescription)>,
}

#[derive(Debug, Clone)]
pub struct UiBuilderEmissionStep {
    description: Vec<(u8, UiLayoutNode, StyleDescription)>,
}

impl UiBuilder {
    pub fn start_layout() -> UiBuilderLayoutStep {
        UiBuilderLayoutStep {
            layout: Vec::new(),
            current_indent: 0u8,
        }
    }
}

impl UiBuilderLayoutStep {
    pub fn node(self, name: &str, tags: &[&str]) -> Self {
        let mut layout = self.layout;
        layout.push((
            self.current_indent,
            UiLayoutNode {
                name: name.into(),
                tags: tags.iter().map(|s| s.to_string()).collect(),
            },
        ));
        Self {
            current_indent: self.current_indent,
            layout: layout,
        }
    }

    pub fn start_children(self) -> Self {
        if let Some((indent, _)) = self.layout.last() {
            if indent < &self.current_indent && self.current_indent - indent > 1 {
                panic!("Cannot indent further than once! Make sure you add at least one node between calls to `start_children`");
            }
        }
        Self {
            layout: self.layout,
            current_indent: self.current_indent + 1,
        }
    }
    pub fn end_children(self) -> Self {
        if self.current_indent <= 0 {
            panic!("Cannot reduce indent past zero! There should only be one root node anyway");
        }
        Self {
            layout: self.layout,
            current_indent: self.current_indent - 1,
        }
    }

    pub fn finish_layout(self) -> UiBuilderDescriptionStep {
        UiBuilderDescriptionStep {
            description: self
                .layout
                .into_iter()
                .map(|v| (v.0, v.1, StyleDescription::default()))
                .collect(),
        }
    }
}

impl UiBuilderDescriptionStep {
    pub fn by_name(&mut self, name: &str, style: StyleDescription) -> &mut Self {
        self.description = self
            .description
            .iter()
            .map(|en| {
                if en.1.name == name {
                    (en.0, en.1.clone(), en.2.clone().apply(&style))
                } else {
                    (en.0, en.1.clone(), en.2.clone())
                }
            })
            .collect();
        self
    }
    pub fn by_tag(&mut self, tag: &[&str], style: StyleDescription) -> &mut Self {
        let tags: Vec<String> = tag.iter().map(|s| s.to_string()).collect();
        self.description = self
            .description
            .iter()
            .map(move |(indent, node, base_style)| {
                for tag2 in &tags {
                    for tag1 in node.tags.iter() {
                        if tag2 == tag1 {
                            return (*indent, node.clone(), base_style.clone().apply(&style));
                        }
                    }
                }
                (*indent, node.clone(), base_style.clone())
            })
            .collect();
        self
    }

    pub fn finish_design(self) -> UiBuilderEmissionStep {
        UiBuilderEmissionStep {
            description: self.description,
        }
    }
}
