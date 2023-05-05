use std::collections::HashMap;
use std::fmt;

use crate::core::internal::IDeviceInfoBuilder;

use crate::plugins::{macos::plugin::MacOSBuilder, windows::plugin::WindowsBuilder};

pub struct DeviceInfoMainBuilder {
    // Key: Name
    // Value
    pub components: HashMap<String, String>,
}

impl IDeviceInfoBuilder for DeviceInfoMainBuilder {
    fn get_components(&self) -> &HashMap<String, String> {
        &self.components
    }

    fn get_components_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.components
    }
}

impl fmt::Display for DeviceInfoMainBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .get_components()
                .iter()
                .map(|component| format!("{}: {}", component.0, component.1))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl DeviceInfoMainBuilder {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn add_user_name(&mut self) -> &mut Self {
        self.add_component("userName", whoami::username().as_str());
        self
    }

    pub fn add_device_name(&mut self) -> &mut Self {
        self.add_component("deviceName", whoami::devicename().as_str());
        self
    }

    pub fn add_platform_name(&mut self) -> &mut Self {
        self.add_component("platform", whoami::platform().to_string().as_str());
        self
    }

    pub fn add_os_distro(&mut self) -> &mut Self {
        self.add_component("osDistro", whoami::distro().as_str());
        self
    }

    pub fn add_cpu_arch(&mut self) -> &mut Self {
        self.add_component("cpuArch", whoami::arch().to_string().as_str());
        self
    }

    #[allow(dead_code)]
    pub fn on_windows<F>(&mut self, _on_windows_plugin: F) -> &mut Self
    where
        F: Fn(&mut WindowsBuilder) -> &mut WindowsBuilder,
    {
        match whoami::platform() == whoami::Platform::Windows {
            true => {
                let windows_builder = WindowsBuilder::new();
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
        F: Fn(&mut MacOSBuilder) -> &mut MacOSBuilder,
    {
        match whoami::platform() == whoami::Platform::MacOS {
            true => {
                let mut macos_builder = MacOSBuilder::new();
                on_macos_plugin(&mut macos_builder);
                self.extend_components(&macos_builder.components);
                self
            }
            false => self,
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self.components).expect("Failed to serialize MachineCodeBuilder")
    }
}
