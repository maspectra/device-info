use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, hash::Hash};

pub trait IDeviceInfoBuilder<KT>
where
    KT: Clone + Copy + Hash + Eq + fmt::Display + Ord,
{
    fn get_components(&self) -> &HashMap<KT, String>;
    fn get_components_mut(&mut self) -> &mut HashMap<KT, String>;

    fn add_component(&mut self, name: &KT, value: &str) -> &mut Self {
        let components = self.get_components_mut();
        if components.get(name).is_some() {
            panic!("Component with name '{}' already exists", name);
        }
        components.insert(*name, value.to_string());
        self
    }

    fn extend_components(&mut self, components: &HashMap<KT, String>) -> &mut Self {
        for component in components {
            self.add_component(component.0, component.1);
        }
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseDeviceInfoBuilder<KT>
where
    KT: Clone + Copy + Hash + Eq + fmt::Display + Ord,
{
    pub components: HashMap<KT, String>,
}

impl<KT> BaseDeviceInfoBuilder<KT>
where
    KT: Clone + Copy + Hash + Eq + fmt::Display + Ord,
{
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
}

impl<KT> Default for BaseDeviceInfoBuilder<KT>
where
    KT: Clone + Copy + Hash + Eq + fmt::Display + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<KT> fmt::Display for BaseDeviceInfoBuilder<KT>
where
    KT: Clone + Copy + Hash + Eq + fmt::Display + Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .get_components()
                .iter()
                .sorted_by_key(|el| el.0)
                .map(|component| format!("{}: {}", component.0, component.1))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl<KT> IDeviceInfoBuilder<KT> for BaseDeviceInfoBuilder<KT>
where
    KT: Clone + Copy + Hash + Eq + fmt::Display + Ord,
{
    fn get_components(&self) -> &HashMap<KT, String> {
        &self.components
    }

    fn get_components_mut(&mut self) -> &mut HashMap<KT, String> {
        &mut self.components
    }
}
