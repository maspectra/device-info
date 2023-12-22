use std::{collections::HashMap, fmt};

use serde;
#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

use crate::core::internal::{BaseDeviceInfoBuilder, IDeviceInfoBuilder};
#[cfg(target_os = "windows")]
use crate::core::string_tools::strip_trailing_newline;
#[cfg(target_os = "windows")]
use crate::plugins::windows::wmi::WmiSingleton;

#[allow(dead_code)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum WindowsBuilderComponents {
    LogonUserName,
    SystemDriveSerialNumber,
    MotherBoardSerialNumber,
    SystemUuid,
    MACAddress,
    ProcessorId,
    Guid,
}

impl serde::Serialize for WindowsBuilderComponents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_string().serialize(serializer)
    }
}

impl WindowsBuilderComponents {
    pub fn as_string(&self) -> String {
        match *self {
            WindowsBuilderComponents::LogonUserName => "logonUserName".to_string(),
            WindowsBuilderComponents::SystemDriveSerialNumber => {
                "systemDriveSerialNumber".to_string()
            }
            WindowsBuilderComponents::MotherBoardSerialNumber => {
                "motherBoardSerialNumber".to_string()
            }
            WindowsBuilderComponents::SystemUuid => "systemUuid".to_string(),
            WindowsBuilderComponents::MACAddress => "MACAddress".to_string(),
            WindowsBuilderComponents::ProcessorId => "processorId".to_string(),
            WindowsBuilderComponents::Guid => "guid".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "logonUserName" => Some(WindowsBuilderComponents::LogonUserName),
            "systemDriveSerialNumber" => Some(WindowsBuilderComponents::SystemDriveSerialNumber),
            "motherBoardSerialNumber" => Some(WindowsBuilderComponents::MotherBoardSerialNumber),
            "systemUuid" => Some(WindowsBuilderComponents::SystemUuid),
            "MACAddress" => Some(WindowsBuilderComponents::MACAddress),
            "processorId" => Some(WindowsBuilderComponents::ProcessorId),
            "guid" => Some(WindowsBuilderComponents::Guid),
            _ => None,
        }
    }
}
impl fmt::Display for WindowsBuilderComponents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_string().as_str())
    }
}

pub trait IWindowsBuilder: IDeviceInfoBuilder<WindowsBuilderComponents> {
    fn add_logon_user_name(&mut self) -> &mut Self;
    fn add_system_drive_serial_number(&mut self) -> &mut Self;
    fn add_mother_board_serial_number(&mut self) -> &mut Self;
    fn add_system_uuid(&mut self) -> &mut Self;
    fn add_mac_address(&mut self) -> &mut Self;
    fn add_processor_id(&mut self) -> &mut Self;
    fn add_machine_guid(&mut self) -> &mut Self;
}

pub struct WindowsBuilder {
    _base: BaseDeviceInfoBuilder<WindowsBuilderComponents>,
}

impl WindowsBuilder {
    pub fn new() -> Self {
        Self {
            _base: BaseDeviceInfoBuilder::<WindowsBuilderComponents>::new(),
        }
    }
}

impl IDeviceInfoBuilder<WindowsBuilderComponents> for WindowsBuilder {
    fn get_components(&self) -> &HashMap<WindowsBuilderComponents, String> {
        &self._base.components
    }
    fn get_components_mut(&mut self) -> &mut HashMap<WindowsBuilderComponents, String> {
        &mut self._base.components
    }
}

#[cfg(target_os = "windows")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UserNameQueryResult {
    user_name: String,
}

#[cfg(target_os = "windows")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SerialNumberQueryResult {
    serial_number: String,
}

#[cfg(target_os = "windows")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct UUIDQueryResult {
    uuid: String,
}

#[cfg(target_os = "windows")]
#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
struct MACAddressQueryResult {
    MACAddress: String,
}

#[cfg(target_os = "windows")]
#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
struct ProcessorIdQueryResult {
    ProcessorId: String,
}

impl IWindowsBuilder for WindowsBuilder {
    fn add_logon_user_name(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let res: Vec<UserNameQueryResult> = WmiSingleton::raw_query(
                "SELECT UserName FROM Win32_ComputerSystem WHERE UserName IS NOT NULL",
            );
            self.add_component(
                &WindowsBuilderComponents::LogonUserName,
                strip_trailing_newline(
                    res.get(0)
                        .and_then(|r| Some(r.user_name.as_str()))
                        .unwrap_or("")
                        .trim(),
                ),
            );
            self
        }

        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }
    fn add_system_drive_serial_number(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let res: Vec<SerialNumberQueryResult> = WmiSingleton::raw_query(
                "SELECT SerialNumber FROM Win32_PhysicalMedia WHERE SerialNumber IS NOT NULL",
            );

            self.add_component(
                &WindowsBuilderComponents::SystemDriveSerialNumber,
                strip_trailing_newline(
                    res.get(0)
                        .and_then(|r| Some(r.serial_number.as_str()))
                        .unwrap_or("")
                        .trim(),
                ),
            );
            self
        }

        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }

    fn add_mother_board_serial_number(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let res: Vec<SerialNumberQueryResult> = WmiSingleton::raw_query(
                "SELECT SerialNumber FROM Win32_BaseBoard WHERE SerialNumber IS NOT NULL",
            );

            self.add_component(
                &WindowsBuilderComponents::MotherBoardSerialNumber,
                strip_trailing_newline(
                    res.get(0)
                        .and_then(|r| Some(r.serial_number.as_str()))
                        .unwrap_or("")
                        .trim(),
                ),
            );
            self
        }
        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }

    fn add_system_uuid(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let res: Vec<UUIDQueryResult> = WmiSingleton::raw_query(
                "SELECT UUID FROM Win32_ComputerSystemProduct WHERE UUID IS NOT NULL",
            );

            self.add_component(
                &WindowsBuilderComponents::SystemUuid,
                strip_trailing_newline(
                    res.get(0)
                        .and_then(|r| Some(r.uuid.as_str()))
                        .unwrap_or("")
                        .trim(),
                ),
            );
            self
        }
        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }

    fn add_mac_address(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let res: Vec<MACAddressQueryResult> = WmiSingleton::raw_query(
                "SELECT MACAddress FROM Win32_NetworkAdapter WHERE MACAddress IS NOT NULL",
            );

            self.add_component(
                &WindowsBuilderComponents::MACAddress,
                strip_trailing_newline(strip_trailing_newline(
                    res.get(0)
                        .and_then(|r| Some(r.MACAddress.as_str()))
                        .unwrap_or("")
                        .trim(),
                )),
            );
            self
        }
        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }

    fn add_processor_id(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let res: Vec<ProcessorIdQueryResult> = WmiSingleton::raw_query(
                "SELECT ProcessorId FROM Win32_Processor WHERE ProcessorId IS NOT NULL",
            );

            self.add_component(
                &WindowsBuilderComponents::ProcessorId,
                strip_trailing_newline(strip_trailing_newline(
                    res.get(0)
                        .and_then(|r| Some(r.ProcessorId.as_str()))
                        .unwrap_or("")
                        .trim(),
                )),
            );
            self
        }
        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }

    fn add_machine_guid(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let rkey = RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey("SOFTWARE\\Microsoft\\Cryptography")
                .unwrap();

            let id: String = rkey.get_value("MachineGuid").unwrap();

            self.add_component(&WindowsBuilderComponents::Guid, id.as_str());
            self
        }
        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }
}

impl Default for WindowsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use super::*;

    #[test]
    fn test_windows_builder_serial_number() {
        let mut builder = WindowsBuilder::new();
        builder.add_system_drive_serial_number();

        println!("{:?}", builder.get_components());
    }

    #[test]
    fn test_windows_builder_mother_board_serial_number() {
        let mut builder = WindowsBuilder::new();
        builder.add_mother_board_serial_number();

        println!("{:?}", builder.get_components());
    }

    #[test]
    fn test_windows_builder_system_uuid() {
        let mut builder = WindowsBuilder::new();
        builder.add_system_uuid();

        println!("{:?}", builder.get_components());
    }

    #[test]
    fn test_windows_builder_mac_address() {
        let mut builder = WindowsBuilder::new();
        builder.add_mac_address();

        println!("{:?}", builder.get_components());
    }

    #[test]
    fn test_windows_builder_logon_user_name() {
        let mut builder = WindowsBuilder::new();
        builder.add_logon_user_name();

        println!("{:?}", builder.get_components());
    }

    #[test]
    fn test_windows_builder_all() {
        let mut builder = WindowsBuilder::new();
        builder
            .add_logon_user_name()
            .add_system_drive_serial_number()
            .add_mother_board_serial_number()
            .add_system_uuid()
            .add_mac_address();

        println!("{:?}", builder.get_components());
    }
}
