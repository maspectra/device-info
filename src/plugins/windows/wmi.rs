use serde::de::DeserializeOwned;

use wmi::{COMLibrary, WMIConnection};

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
        con.raw_query::<T>(query)
            .expect(format!("Failed to execute query {}", query).as_str())
    }
}
