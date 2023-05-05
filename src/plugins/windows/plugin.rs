// use std::collections::HashMap;

// use crate::core::internal::IDeviceInfoBuilder;

// // pub struct WmiSystemDriveSerialNumberMachineCodeComponent {}

// pub struct WindowsBuilder {
//     pub components: HashMap<String, String>,
// }

// impl IDeviceInfoBuilder for WindowsBuilder {
//     fn get_components(&self) -> &HashMap<String, String> {
//         &self.components
//     }

//     fn get_components_mut(&mut self) -> &mut HashMap<String, String> {
//         &mut self.components
//     }
// }

// impl WindowsBuilder {
//     pub fn new() -> Self {
//         Self {
//             components: HashMap::new(),
//         }
//     }
// }

// impl Default for WindowsBuilder {
//     fn default() -> Self {
//         Self::new()
//     }
// }
