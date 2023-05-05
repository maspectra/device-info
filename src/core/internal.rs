use std::collections::HashMap;

pub trait IDeviceInfoBuilder {
    fn get_components(&self) -> &HashMap<String, String>;
    fn get_components_mut(&mut self) -> &mut HashMap<String, String>;

    fn add_component(&mut self, name: &str, value: &str) -> &mut Self {
        let components = self.get_components_mut();
        if components.get(name).is_some() {
            panic!("Component with name '{}' already exists", name);
        }
        components.insert(name.to_string(), value.to_string());
        self
    }

    fn extend_components(&mut self, components: &HashMap<String, String>) -> &mut Self {
        for component in components {
            self.add_component(component.0, component.1);
        }
        self
    }
}
