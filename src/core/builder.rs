use std::collections::HashMap;
use std::fmt;

use itertools::Itertools;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};

use crate::core::internal::IDeviceInfoBuilder;
use crate::plugins::macos::plugin::{MacOSBuilder, MacOSBuilderComponents};
use crate::plugins::windows::plugin::{WindowsBuilder, WindowsBuilderComponents};

use super::internal::BaseDeviceInfoBuilder;

// use crate::plugins::{macos::plugin, windows::plugin::WindowsBuilder};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum MainBuilderComponents {
    UserName,
    DeviceName,
    OSPlatform,
    OSDistro,
    CpuArch,
    WindowsBuilderComponents(WindowsBuilderComponents),
    MacOSBuilderComponents(MacOSBuilderComponents),
}

impl MainBuilderComponents {
    pub fn as_string(&self) -> String {
        match *self {
            MainBuilderComponents::UserName => "userName".to_string(),
            MainBuilderComponents::DeviceName => "deviceName".to_string(),
            MainBuilderComponents::OSPlatform => "osPlatform".to_string(),
            MainBuilderComponents::OSDistro => "osDistro".to_string(),
            MainBuilderComponents::CpuArch => "cpuArch".to_string(),
            MainBuilderComponents::WindowsBuilderComponents(ref component) => {
                format!("Windows::{}", component.as_string())
            }
            MainBuilderComponents::MacOSBuilderComponents(ref component) => {
                format!("MacOS::{}", component.as_string())
            }
        }
    }
}

impl Serialize for MainBuilderComponents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MainBuilderComponents {
    fn deserialize<D>(deserializer: D) -> Result<MainBuilderComponents, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "userName" => Ok(MainBuilderComponents::UserName),
            "deviceName" => Ok(MainBuilderComponents::DeviceName),
            "osPlatform" => Ok(MainBuilderComponents::OSPlatform),
            "osDistro" => Ok(MainBuilderComponents::OSDistro),
            "cpuArch" => Ok(MainBuilderComponents::CpuArch),
            _ if s.starts_with("Windows::") => {
                let component = s.strip_prefix("Windows::").unwrap();
                match WindowsBuilderComponents::from_str(component) {
                    Some(v) => Ok(MainBuilderComponents::WindowsBuilderComponents(v)),
                    None => Err(serde::de::Error::custom(format!(
                        "Invalid WindowsBuilderComponents: {}",
                        s
                    ))),
                }
            }
            _ if s.starts_with("MacOS::") => {
                let component = s.strip_prefix("MacOS::").unwrap();
                match MacOSBuilderComponents::from_str(component) {
                    Some(v) => Ok(MainBuilderComponents::MacOSBuilderComponents(v)),
                    None => Err(serde::de::Error::custom(format!(
                        "Invalid MacOSBuilderComponents: {}",
                        s
                    ))),
                }
            }
            _ => Err(serde::de::Error::custom(format!(
                "Invalid MainBuilderComponents: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for MainBuilderComponents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_string().as_str())
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
            &MainBuilderComponents::OSPlatform,
            whoami::platform().to_string().as_str(),
        );
        self
    }

    fn add_os_distro(&mut self) -> &mut Self {
        self.add_component(&MainBuilderComponents::OSDistro, whoami::distro().as_str());
        self
    }

    fn add_cpu_arch(&mut self) -> &mut Self {
        self.add_component(
            &MainBuilderComponents::CpuArch,
            whoami::arch().to_string().as_str(),
        );
        self
    }

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
        let mut map = serializer.serialize_map(Some(self._base.components.len()))?;
        for (k, v) in self._base.components.iter().sorted_by_key(|el| el.0) {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for MainDeviceInfoBuilder {
    fn deserialize<D>(deserializer: D) -> Result<MainDeviceInfoBuilder, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let components = HashMap::<MainBuilderComponents, String>::deserialize(deserializer)?;
        Ok(Self {
            _base: BaseDeviceInfoBuilder::<MainBuilderComponents> { components },
        })
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

    #[test]
    fn test_main_builder_deserialize() {
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
        let serialized = serde_json::to_string(&builder).unwrap();

        let deserialized: MainDeviceInfoBuilder = match serde_json::from_str(&serialized) {
            Ok(v) => v,
            Err(e) => panic!("Invalid from_str: {}", e),
        };
        println!("{:?}", deserialized.get_components());
    }
}
