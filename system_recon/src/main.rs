// TODO:
// 1. Implment windows registry user match
// 2. Get number of actual users
// 3. Get execution policy
// 4. Get network adapters

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
    domain: String,
    product_version: String,
    os_version: String,
    execution_policy: String

}

fn overall_info() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion").unwrap();
    let active_computer_name = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ActiveComputerName").unwrap();
    let powershell = hklm.open_subkey("SOFTWARE\\Microsoft\\PowerShell\\1\\ShellIds\\Microsoft.PowerShell").unwrap();
    let tcpip_params = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters").unwrap();

    let domain: String = tcpip_params.get_value("Domain").unwrap();

    let domain = if domain.is_empty() == true {
        "System is not in a domain".to_string()
    } else {
        domain
    };

    let display_version: String = cur_ver.get_value("DisplayVersion").unwrap();
    let release_id: String = cur_ver.get_value("ReleaseId").unwrap();

    let info = ComputerInfo {
        computer_name: active_computer_name.get_value("ComputerName").unwrap(),
        domain: domain,
        product_version: cur_ver.get_value("ProductName").unwrap(),
        os_version: format!("{} {}", display_version, release_id),
        execution_policy: powershell.get_value("ExecutionPolicy").unwrap()

    };


    let info_json = serde_json::to_string_pretty(&info).unwrap();

    return info_json
}

#[derive(Serialize, Deserialize)]
struct Autorun {
    keyname: String,
    keyvalue: String
}

fn autorun_programs() -> String {
    let set_as_run = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run").unwrap();

    let mut autorun_output = vec![];

    for (name, value) in set_as_run.enum_values().map(|x| x.unwrap()) {
        let value = Autorun {
            keyname: name,
            keyvalue: value.to_string()
        };

        autorun_output.push(value)
    }

    let autorun_json = serde_json::to_string_pretty(&autorun_output).unwrap();

    return autorun_json
}

fn user_info() {

}

fn network_info() {

}