use serde::de::DeserializeOwned;
#[cfg(target_os = "windows")]
use wmi::{COMLibrary, WMIConnection};

#[cfg(target_os = "windows")]
thread_local! {
    static COM_LIB: COMLibrary = COMLibrary::without_security().unwrap();
}

pub struct WmiSingleton;

impl WmiSingleton {
    pub fn raw_query<T>(query: &str) -> Vec<T>
    where
        T: DeserializeOwned,
    {
        let con = WMIConnection::new(COM_LIB.with(|con| *con)).expect("Failed to connect to WMI");
        con.raw_query::<T>(query).expect("Failed to execute query")
    }
}
