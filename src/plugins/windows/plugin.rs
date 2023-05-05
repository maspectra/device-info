use std::collections::HashMap;

// pub struct WmiSystemDriveSerialNumberMachineCodeComponent {}

pub struct WindowsBuilderPlugin {
    pub components: HashMap<String, String>,
}

impl WindowsBuilderPlugin {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
}
