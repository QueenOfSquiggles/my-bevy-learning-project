use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;
use style_description::StyleDescription;

pub mod style_description;
#[derive(Default, Debug, Clone)]
pub struct LayoutNode {
    name: String,
    tags: Vec<String>,
    children: Vec<LayoutNode>,
}

pub trait IntoLayoutNode<T>: Sized {
    fn to_layout_node(self) -> LayoutNode;
}

#[derive(Clone, Debug)]
pub enum Gui {
    Node(&'static str, &'static [&'static str]),
    Parent(&'static str, &'static [&'static str], &'static [Gui]),
}

impl IntoLayoutNode<Gui> for Gui {
    fn to_layout_node(self) -> LayoutNode {
        match self {
            Gui::Node(name, tags) => LayoutNode {
                name: name.into(),
                tags: tags.to_vec().iter().map(|s| s.to_string()).collect(),
                children: Vec::new(),
            },
            Gui::Parent(name, tags, children) => {
                let mut parent = Gui::Node(name, tags).to_layout_node();
                for child in children {
                    parent.children.push(child.clone().to_layout_node());
                }
                parent
            }
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct GuiBuilder {
    root: LayoutNode,
    theme: StyleDescription,
    style_named: HashMap<String, StyleDescription>,
    style_tagged: HashMap<String, StyleDescription>,
}

impl GuiBuilder {
    pub fn new(gui: Gui) -> Self {
        Self {
            root: gui.to_layout_node(),
            ..default()
        }
    }

    pub fn style_all(mut self, style: StyleDescription) -> Self {
        self.theme.apply(&style);
        self
    }

    pub fn style_by_name(mut self, name: &str, style: StyleDescription) -> Self {
        if let Some(s) = self.style_named.get_mut(name) {
            s.apply(&style);
        } else {
            self.style_named.insert(name.into(), style);
        }
        self
    }
    pub fn style_by_tag(mut self, name: &str, style: StyleDescription) -> Self {
        if let Some(s) = self.style_named.get_mut(name) {
            s.apply(&style);
        } else {
            self.style_tagged.insert(name.into(), style);
        }
        self
    }

    pub fn emit<'a, 'b>(self, mut command: Commands<'a, 'b>) -> GuiBuilderEmission<'a, 'b>
    where
        'a: 'b,
    {
        let mut nodes = Vec::new();
        let mut queue: VecDeque<(LayoutNode, Option<Entity>)> = VecDeque::new();
        queue.push_back((self.root, None));
        while !queue.is_empty() {
            let Some((node, parent)) = queue.pop_front() else {
                break;
            };
            let mut style = self.theme.clone();
            for t in node.tags.clone() {
                if let Some(s) = self.style_tagged.get(&t) {
                    style.apply(s);
                }
            }
            if let Some(s) = self.style_named.get(&node.name) {
                style.apply(s);
            }
            let mut e = command.spawn(NodeBundle::default());
            if let Some(p) = parent {
                e.set_parent(p);
            }
            let handle = e.id();

            for c in node.children {
                queue.push_back((c, Some(handle)));
            }
            nodes.push(EmissionNode {
                name: node.name,
                tags: node.tags,
                handle,
                style,
            })
        }

        GuiBuilderEmission {
            nodes,
            cmd: Some(command),
        }
    }
}
#[derive(Clone, Debug)]
struct EmissionNode {
    name: String,
    tags: Vec<String>,
    handle: Entity,
    style: StyleDescription,
}
pub struct GuiBuilderEmission<'a, 'b> {
    nodes: Vec<EmissionNode>,
    cmd: Option<Commands<'a, 'b>>,
}

impl GuiBuilderEmission<'_, '_> {
    pub fn bundle_by_name<B>(&mut self, name: &str, bundle: B) -> &mut Self
    where
        B: Bundle + Clone,
    {
        let nodes = self
            .nodes
            .iter()
            .filter(|p| p.name == name)
            .cloned()
            .collect();
        if let Some(cmd) = &mut self.cmd {
            Self::bundle_targets(nodes, cmd, bundle);
        }
        self
    }
    pub fn bundle_by_tag<B>(&mut self, tag: &str, bundle: B) -> &mut Self
    where
        B: Bundle + Clone,
    {
        let nodes = self
            .nodes
            .iter()
            .filter(|p| p.tags.iter().any(|p| p == tag))
            .cloned()
            .collect();
        if let Some(cmd) = &mut self.cmd {
            Self::bundle_targets(nodes, cmd, bundle);
        }
        self
    }

    fn bundle_targets<B>(nodes: Vec<EmissionNode>, commands: &mut Commands, bundle: B)
    where
        B: Bundle + Clone,
    {
        for n in nodes {
            if let Some(ecmd) = &mut commands.get_entity(n.handle) {
                ecmd.insert(bundle.clone());
            }
        }
    }

    pub fn finish(&mut self) {
        if let Some(commands) = &mut self.cmd {
            for n in self.nodes.iter() {
                if let Some(ecmd) = &mut commands.get_entity(n.handle) {
                    ecmd.insert(Into::<Style>::into(n.style.clone()));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GUI_LAYOUT: Gui = {
        use super::Gui::*;

        let parent = Parent(
            "root",
            &[],
            &[Node("a", &["btn"]), Node("c", &[]), Node("c", &[])],
        );
        parent
    };

    fn setup_gui(commands: Commands) {
        let layout = GUI_LAYOUT;
        GuiBuilder::new(layout)
            .style_all(StyleDescription {
                width: Some(Val::Percent(50.)),
                height: Some(Val::Percent(50.)),
                ..default()
            })
            .emit(commands)
            .bundle_by_tag("btn", ButtonBundle::default())
            .finish();
    }

    fn prep_test() -> App {
        let mut app = App::new();
        app.add_systems(Startup, setup_gui);
        app.update();
        app
    }

    #[test]
    fn test_display_structure() {
        println!("{:#?}", GUI_LAYOUT);
    }

    #[test]
    fn test_nodes_present() {
        let mut app = prep_test();
        assert_eq!(app.world.query::<&Node>().iter(&app.world).count(), 4);
    }
    #[test]
    fn test_button_present() {
        let mut app = prep_test();
        assert_eq!(app.world.query::<&Button>().iter(&app.world).count(), 1);
    }
}
