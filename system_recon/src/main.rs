use winreg::enums::*;
use winreg::RegKey;
use serde::{Serialize, Deserialize};
use serde_json::json_internal_vec;
use sysinfo::*;
use clap::{Command, Arg};
use std::env::current_exe;
use std::process::{Command as process_command, Stdio};
use std::str;
use execute::Execute;
use quickxml_to_serde::{xml_string_to_json, Config};
//use winapi::um::winsvc::{GetServiceDisplayNameA};



// Custom modules
mod windows_utf16_convert;
use windows_utf16_convert::{parse_utf16_bytes};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let matches = Command::new("system_recon")
    .author("By: Hunter Pittman")
    .about("This script pulls basic recon information from a target system and outputs it in a way another related tool can parse")
    .arg(Arg::new("overall")
        .long("overall")
        .short('o')
        .required(false)
        .takes_value(false)
        .help("Displays general info about the system"))
    .arg(Arg::new("autorun")
        .long("autorun")
        .short('a')
        .required(false)
        .takes_value(false)
        .help("Lists all autorunning programs on the system"))
    .arg(Arg::new("network")
        .long("network")
        .short('n')
        .required(false)
        .takes_value(false)
        .help("Lists all network adapters and transfer totals"))
    .arg(Arg::new("users")
        .long("users")
        .short('u')
        .required(false)
        .takes_value(false)
        .help("Lists all users and groups on the system"))
    .arg(Arg::new("processes")
        .long("processes")
        .short('p')
        .required(false)
        .takes_value(false)
        .help("Dumps a list of processes on the system"))
    .arg(Arg::new("sysmon")
        .long("sysmon")
        .short('s')
        .required(false)
        .takes_value(false)
        .help("Configures sysmon on the system"))
    .get_matches();


    if  matches.is_present("overall") {
        println!("{}", serde_json::to_string_pretty(&overall_info()).unwrap());
    } 
    
    if matches.is_present("autorun") {
        println!("{:?}", serde_json::to_string_pretty(&autorun_programs()).unwrap());
    }

    if matches.is_present("network") {
        println!("{}", serde_json::to_string_pretty(&adapter_info(&sys)).unwrap());
    }

    if matches.is_present("users") {
        println!("{}", serde_json::to_string_pretty(&user_info(&sys)).unwrap());
    }

    if matches.is_present("processes") {
        println!("{}", serde_json::to_string_pretty(&process_info(&sys)).unwrap());
    }

    if matches.is_present("sysmon") {
        println!("{}", serde_json::to_string_pretty(&configure_sysmon(&sys)).unwrap());
    }

}

#[derive(Serialize, Deserialize)]
struct ComputerInfo {
    computer_name: String,
    domain: String,
    product_version: String,
    os_version: String,
    execution_policy: String

}

fn overall_info() -> ComputerInfo {
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

    return info

}

fn autorun_programs() -> serde_json::Value {
    // Check where sysinternals is developer vs release
    let full_exe_path = current_exe().unwrap();

    let mut split_exe_path: Vec<&str> = Vec::new();
    
    if full_exe_path.to_str().unwrap().contains("target") {
        let temp: Vec<&str> = full_exe_path.to_str().unwrap().split("system_recon\\target\\debug\\system_recon.exe").collect();
        split_exe_path.push(temp[0]);
    } else {
        let temp: Vec<&str> = full_exe_path.to_str().unwrap().split("system_recon.exe").collect();
        split_exe_path.push(temp[0]);
    };

    let partial_exe_path = split_exe_path[0].to_string();
    let sysinternals_exe_string = partial_exe_path + &"SysinternalsSuite\\Autorunsc64.exe".to_string();

    let mut command = process_command::new(sysinternals_exe_string);
    command.arg("-nobanner");
    command.arg("-accepteula");
    command.arg("-u");
    command.arg("-v"); // Query VirusTotal for malware based on file hash. Add 'r' to open reports for files with non-zero detection. Files reported as not previously scanned will be uploaded to VirusTotal if the 's' option is specified. Note scan results may not be available for five or more minutes.
    command.arg("-vt"); // Before using VirusTotal features, you must accept the VirusTotal terms of service. If you haven't accepted the terms and you omit this option, you will be interactively prompted.
    command.arg("-x"); // Specifies xml format
    command.arg("-a");
    command.arg("tbmshl");
    command.arg("*"); //

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let output = command.execute_output().unwrap();
    
    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("Ok.");
        } else {
            eprintln!("Failed.");
        }
    } else {
        eprintln!("Interrupted!");
    }

    let xml_data =  parse_utf16_bytes(output.stdout.as_slice()).expect("Bruh it brokey");

    let conf = Config::new_with_defaults();
    let json = xml_string_to_json(xml_data.to_owned(), &conf);

    return json.unwrap()
}


#[derive(Serialize, Deserialize)]
struct User {
    uid: u32,
    gid: u32,
    name: String,
    groups: Vec<String>
}


fn user_info(sys: &System) -> std::vec::Vec<User> {

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

    return users

}

#[derive(Serialize, Deserialize)]
struct NetworkInterface {
    interface_name: String,
    total_transmitted_packets: u64
}

fn adapter_info(sys: &System) -> std::vec::Vec<NetworkInterface> {
    let networks = sys.networks();

    let mut network_interfaces = vec![];

    for (interface_name, data) in networks {
        let value = NetworkInterface {
            interface_name: interface_name.to_string(),
            total_transmitted_packets: data.total_packets_transmitted()
        };

        network_interfaces.push(value);
    }

    return network_interfaces
}

#[derive(Serialize, Deserialize)]
struct Process {
    pid: u32,
    parent_process: u32,
    name: String,
    command: Vec<String>

}

fn process_info(sys: &System) -> std::vec::Vec<Process> {
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

    return process_dump
}

fn configure_sysmon(sys: &System) -> String {
    let processes = process_info(sys);


    
    
    return "bruh".to_string()
}

fn settings_set() -> String {
    

    return "bruh".to_string()
}