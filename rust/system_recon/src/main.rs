// TODO:
// 1. Implment windows registry user match
// 2. Get number of actual users
// 3. Get logon server
// 4. Get execution policy
// 5. Get network adapters

use winreg::enums::*;
use winreg::RegKey;
use serde::{Serialize, Deserialize};

fn main() {
    let x = overall_info();
    println!("{}", x)
}

#[derive(Serialize, Deserialize)]
struct ComputerInfo {
    computer_name: String,
    product_version: String,
    os_version: String,
    user_name: String,
    number_of_users: u8,
    logon_server: String,
    execution_policy: String

}

fn overall_info() -> std::string::String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion").unwrap();
    let active_computer_name = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ActiveComputerName").unwrap();
    
    let display_version: String = cur_ver.get_value("DisplayVersion").unwrap();
    let release_id: String = cur_ver.get_value("ReleaseId").unwrap();

    let info = ComputerInfo {
        computer_name: active_computer_name.get_value("ComputerName").unwrap(),
        product_version: cur_ver.get_value("ProductName").unwrap(),
        os_version: format!("{} {}", display_version, release_id),
        user_name: "TBD".to_string(),
        number_of_users: 1,
        logon_server: "TBD".to_string(),
        execution_policy: "TBD".to_string()

    };


    let info_json = serde_json::to_string_pretty(&info).unwrap();

    return info_json
}
