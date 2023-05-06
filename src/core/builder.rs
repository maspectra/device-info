use std::collections::HashMap;
use std::fmt;

use serde::Serialize;

use crate::core::internal::IDeviceInfoBuilder;
use crate::plugins::macos::plugin::{MacOSBuilder, MacOSBuilderComponents};
use crate::plugins::windows::plugin::{WindowsBuilder, WindowsBuilderComponents};

use super::internal::BaseDeviceInfoBuilder;

// use crate::plugins::{macos::plugin, windows::plugin::WindowsBuilder};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum MainBuilderComponents {
    UserName,
    DeviceName,
    Platform,
    OsDistro,
    CpuArch,
    WindowsBuilderComponents(WindowsBuilderComponents),
    MacOSBuilderComponents(MacOSBuilderComponents),
}

impl MainBuilderComponents {
    pub fn as_str(&self) -> &str {
        match *self {
            MainBuilderComponents::UserName => "userName",
            MainBuilderComponents::DeviceName => "deviceName",
            MainBuilderComponents::Platform => "platform",
            MainBuilderComponents::OsDistro => "osDistro",
            MainBuilderComponents::CpuArch => "cpuArch",
            MainBuilderComponents::WindowsBuilderComponents(ref component) => component.as_str(),
            MainBuilderComponents::MacOSBuilderComponents(ref component) => component.as_str(),
        }
    }
}

impl Serialize for MainBuilderComponents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl fmt::Display for MainBuilderComponents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub trait IMainBuilder: IDeviceInfoBuilder<MainBuilderComponents> {
    fn add_user_name(&mut self) -> &mut Self;
    fn add_device_name(&mut self) -> &mut Self;
    fn add_platform_name(&mut self) -> &mut Self;
    fn add_os_distro(&mut self) -> &mut Self;
    fn add_cpu_arch(&mut self) -> &mut Self;

    fn on_windows<F>(&mut self, on_windows_plugin: F) -> &mut Self
    where
        F: Fn(&mut WindowsBuilder) -> &mut WindowsBuilder;

    fn on_macos<F>(&mut self, on_macos_plugin: F) -> &mut Self
    where
        F: Fn(&mut MacOSBuilder) -> &mut MacOSBuilder;
}

pub struct MainDeviceInfoBuilder {
    _base: BaseDeviceInfoBuilder<MainBuilderComponents>,
}

impl MainDeviceInfoBuilder {
    pub fn new() -> Self {
        Self {
            _base: BaseDeviceInfoBuilder::<MainBuilderComponents>::new(),
        }
    }
}

impl fmt::Display for MainDeviceInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._base.fmt(f)
    }
}

impl IDeviceInfoBuilder<MainBuilderComponents> for MainDeviceInfoBuilder {
    fn get_components(&self) -> &HashMap<MainBuilderComponents, String> {
        &self._base.components
    }
    fn get_components_mut(&mut self) -> &mut HashMap<MainBuilderComponents, String> {
        &mut self._base.components
    }
}

impl IMainBuilder for MainDeviceInfoBuilder {
    fn add_user_name(&mut self) -> &mut Self {
        self.add_component(
            &MainBuilderComponents::UserName,
            whoami::username().as_str(),
        );
        self
    }

    fn add_device_name(&mut self) -> &mut Self {
        self.add_component(
            &MainBuilderComponents::DeviceName,
            whoami::devicename().as_str(),
        );
        self
    }

    fn add_platform_name(&mut self) -> &mut Self {
        self.add_component(
            &MainBuilderComponents::Platform,
            whoami::platform().to_string().as_str(),
        );
        self
    }

    fn add_os_distro(&mut self) -> &mut Self {
        self.add_component(&MainBuilderComponents::OsDistro, whoami::distro().as_str());
        self
    }

    fn add_cpu_arch(&mut self) -> &mut Self {
        self.add_component(
            &MainBuilderComponents::CpuArch,
            whoami::arch().to_string().as_str(),
        );
        self
    }

    // #[allow(dead_code)]
    // pub fn on_windows<F>(&mut self, _on_windows_plugin: F) -> &mut Self
    // where
    //     F: Fn(&mut WindowsBuilder) -> &mut WindowsBuilder,
    // {
    //     match whoami::platform() == whoami::Platform::Windows {
    //         true => {
    //             let windows_builder = WindowsBuilder::new();
    //             // on_windows_plugin(&mut windows_builder);
    //             self.extend_components(&windows_builder.components);
    //             self
    //         }
    //         // if is not windows, return self directly
    //         false => self,
    //     }
    // }

    fn on_windows<F>(&mut self, on_windows_plugin: F) -> &mut Self
    where
        F: Fn(&mut WindowsBuilder) -> &mut WindowsBuilder,
    {
        match whoami::platform() == whoami::Platform::Windows {
            true => {
                let mut windows_builder = WindowsBuilder::new();
                on_windows_plugin(&mut windows_builder);
                self.extend_components(
                    &(windows_builder
                        .get_components()
                        .iter()
                        .map(|component| {
                            (
                                MainBuilderComponents::WindowsBuilderComponents(*component.0),
                                component.1.to_owned(),
                            )
                        })
                        .collect()),
                );
                self
            }
            false => self,
        }
    }

    fn on_macos<F>(&mut self, on_macos_plugin: F) -> &mut Self
    where
        F: Fn(&mut MacOSBuilder) -> &mut MacOSBuilder,
    {
        match whoami::platform() == whoami::Platform::MacOS {
            true => {
                let mut macos_builder = MacOSBuilder::new();
                on_macos_plugin(&mut macos_builder);
                self.extend_components(
                    &(macos_builder
                        .get_components()
                        .iter()
                        .map(|component| {
                            (
                                MainBuilderComponents::MacOSBuilderComponents(*component.0),
                                component.1.to_owned(),
                            )
                        })
                        .collect()),
                );
                self
            }
            false => self,
        }
    }
}

impl Serialize for MainDeviceInfoBuilder {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        self._base.serialize(serializer)
    }
}

impl Default for MainDeviceInfoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugins::macos::plugin::IMacOSBuilder;

    #[test]
    fn test_main_builder() {
        let mut builder = MainDeviceInfoBuilder::new();
        builder
            .add_user_name()
            .add_platform_name()
            .add_device_name()
            .add_cpu_arch()
            .add_os_distro()
            .on_macos(|macos_builder| {
                macos_builder
                    .add_platform_serial_number()
                    .add_system_drive_serial_number()
            });
        println!("{:?}", builder.get_components());
    }

    #[test]
    fn test_main_builder_serialize() {
        let mut builder = MainDeviceInfoBuilder::new();
        builder
            .add_user_name()
            .add_platform_name()
            .add_device_name()
            .add_cpu_arch()
            .add_os_distro()
            .on_macos(|macos_builder| {
                macos_builder
                    .add_platform_serial_number()
                    .add_system_drive_serial_number()
            });
        println!("{}", serde_json::to_string(&builder).unwrap());
    }
}
