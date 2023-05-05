use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use crate::plugins::{macos::plugin::MacOSBuilderPlugin, windows::plugin::WindowsBuilderPlugin};

pub struct MachineCodeBuilder {
    // Key: Name
    // Value
    pub components: HashMap<String, String>,
}

impl Display for MachineCodeBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .components
                .clone()
                .into_iter()
                .map(|component| format!("{}: {}", component.0, component.1))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl MachineCodeBuilder {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn add_user_name(&mut self) -> &mut Self {
        self.components
            .insert("UserName".to_string(), whoami::username());
        self
    }

    pub fn add_device_name(&mut self) -> &mut Self {
        self.components
            .insert("DeviceName".to_string(), whoami::devicename());
        self
    }

    pub fn add_platform_name(&mut self) -> &mut Self {
        self.components
            .insert("Platform".to_string(), whoami::platform().to_string());
        self
    }

    pub fn add_os_distro(&mut self) -> &mut Self {
        self.components
            .insert("OSDistro".to_string(), whoami::distro());
        self
    }

    pub fn add_cpu_arch(&mut self) -> &mut Self {
        self.components
            .insert("CpuArch".to_string(), whoami::arch().to_string());
        self
    }

    #[allow(dead_code)]
    pub fn on_windows<F>(&mut self, _on_windows_plugin: F) -> &mut Self
    where
        F: Fn(&mut WindowsBuilderPlugin) -> &mut WindowsBuilderPlugin,
    {
        match whoami::platform() == whoami::Platform::Windows {
            true => {
                let windows_builder = WindowsBuilderPlugin::new();
                // on_windows_plugin(&mut windows_builder);
                self.extend_components(&windows_builder.components);
                self
            }
            // if is not windows, return self directly
            false => self,
        }
    }

    pub fn on_macos<F>(&mut self, on_macos_plugin: F) -> &mut Self
    where
        F: Fn(&mut MacOSBuilderPlugin) -> &mut MacOSBuilderPlugin,
    {
        match whoami::platform() == whoami::Platform::MacOS {
            true => {
                let mut macos_builder = MacOSBuilderPlugin::new();
                on_macos_plugin(&mut macos_builder);
                self.extend_components(&macos_builder.components);
                self
            }
            false => self,
        }
    }

    pub fn add_component(&mut self, name: &str, value: &str) {
        // check if the given name already exists
        if self.components.get(name).is_some() {
            panic!("Component with name '{}' already exists", name);
        }

        self.components.insert(name.to_string(), value.to_string());
    }

    pub fn extend_components(&mut self, components: &HashMap<String, String>) {
        for component in components {
            self.add_component(component.0, component.1);
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self.components).expect("Failed to serialize MachineCodeBuilder")
    }
}
