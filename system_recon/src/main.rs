// TODO:
// DONE!

use std::fmt::Debug;
use winreg::enums::*;
use winreg::RegKey;
use serde::{Serialize, Deserialize};
use serde_json::json_internal_vec;
use sysinfo::*;


fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}", overall_info());
    println!("{}", autorun_programs());
    println!("{}", network_info(&sys));
    println!("{}", user_info(&sys));
    println!("{}", process_info(&sys));
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

#[derive(Serialize, Deserialize)]
struct User {
    uid: u32,
    gid: u32,
    name: String,
    groups: Vec<String>
}


fn user_info(sys: &System) -> String {

    let mut users = vec![];

    for user in sys.users() {
        let value = User {
            uid: *user.uid(),
            gid: *user.gid(),
            name: user.name().to_string(),
            groups: Vec::from(user.groups()),
        };

        users.push(value);
    }

    let users_json = serde_json::to_string_pretty(&users).unwrap();

    return users_json

}

#[derive(Serialize, Deserialize)]
struct NetworkInterface {
    interface_name: String,
    total_transmitted_packets: u64
}


fn network_info(sys: &System) -> String {
    let networks = sys.networks();

    let mut network_interfaces = vec![];

    for (interface_name, data) in networks {
        let value = NetworkInterface {
            interface_name: interface_name.to_string(),
            total_transmitted_packets: data.total_packets_transmitted()
        };

        network_interfaces.push(value);
    }

    let network_interfaces_json = serde_json::to_string_pretty(&network_interfaces).unwrap();

    return network_interfaces_json
}


#[derive(Serialize, Deserialize)]
struct Process {
    pid: u32,
    parent_process: u32,
    name: String,
    command: Vec<String>

}

fn process_info(sys: &System) -> String {
    let processes = sys.processes();

    let mut process_dump = vec![];

    for (pid, process_data) in processes {
        let value = Process {
            pid: pid.as_u32(),
            parent_process: match process_data.parent() {
                None => 0,
                Some(ppid) => ppid.as_u32()
            },
            name: process_data.name().to_string(),
            command: process_data.cmd().to_vec()
        };
        process_dump.push(value);
    }

    let process_dump_json = serde_json::to_string_pretty(&process_dump).unwrap();

    return process_dump_json


}