use clap::{Arg, ArgAction, Command};
use std::process::{Command as CommandProc, exit};
use uzers::get_current_uid;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const BACKUP_FOLDER: &str = "/opt/proxychainsbackup/";
const CONFIG_FILE: &str = "/etc/proxychains.conf";
const CUSTOM_PROXY_FILE: &str = "/opt/proxychainsbackup/custom.lst";

const TOR_PROXY: &str = "socks5 	127.0.0.1 9050";
const CHISEL_PROXY: &str = "socks5 	127.0.0.1 1080";

fn logo() {
    println!(r#"
 _ __  _ __ _____  ___   _  ___| |__   __ _(_)_ __  ___   
| '_ \| '__/ _ \ \/ / | | |/ __| '_ \ / _` | | '_ \/ __|  
| |_) | | | (_) >  <| |_| | (__| | | | (_| | | | | \__ \  
| .__/|_|  \___/_/\_\__,  |\___|_| |_|\__,_|_|_| |_|___/  
|_|                  |___/                                
              Simple Proxychains Changer
                   coded by x0rr

    "#);
}

fn backupconfig() {
    fs::create_dir_all(BACKUP_FOLDER).unwrap_or_else(|_| panic!("Failed to create backup folder."));
    fs::copy(CONFIG_FILE, format!("{}/proxychains.conf", BACKUP_FOLDER))
        .unwrap_or_else(|_| panic!("Failed to backup config file."));
    println!("[+] Backup done.");
}

fn restoreconfig() {
    let backup_path = format!("{}/proxychains.conf", BACKUP_FOLDER);
    if Path::new(&backup_path).exists() {
        fs::copy(&backup_path, CONFIG_FILE).unwrap_or_else(|_| panic!("Failed to restore config file."));
        println!("[+] Config restored.");
    } else {
        println!("[x] Backup not found.");
    }
}

fn enable_proxy(proxy_line: &str) {
    let content = fs::read_to_string(CONFIG_FILE).expect("Failed to read config file.");

    // Uncomment the specific proxy line if it starts with "#"
    let new_content: Vec<String> = content
        .lines()
        .map(|line| {
            // If the line starts with a "#" and contains the proxy line
            if line.trim_start().starts_with("#") && line.contains(proxy_line) {
                // Remove the comment marker "#"
                line.trim_start_matches('#').trim_start().to_string()
            } else {
                line.to_string()  
            }
        })
        .collect();

    // Write the modified content back to the config file
    fs::write(CONFIG_FILE, new_content.join("\n")).expect("Failed to write to config file.");
}

fn check() {
    // Fetch the external IP address using proxychains and curl
    let myip = match CommandProc::new("proxychains")
        .arg("curl")
        .arg("-s")
        .arg("api.ipify.org")
        .output() {
            Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
            Err(_) => {
                eprintln!("Failed to fetch IP address with proxychains.");
                exit(1);  // Exit if the command fails
            }
        };
    // println!("{}", myip.trim());
    // Fetch the geolocation info using the IP address
    let execute = match CommandProc::new("curl")
        .arg("-s")
        .arg(format!("https://ipinfo.io/{}", myip.trim()))
        .output() {
            Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
            Err(_) => {
                eprintln!("Failed to fetch IP info.");
                exit(1);  // Exit if the command fails
            }
        };

    let ip = extract_field(&execute, "ip");
    let country = extract_field(&execute, "country");
    let region = extract_field(&execute, "region");
    let loc = extract_field(&execute, "loc");
    let org = extract_field(&execute, "org");

    // Display the result in a formatted table
    let width = vec![
        format!("IP: {}", ip),
        format!("Country: {}", country),
        format!("Region: {}", region),
        format!("Location: {}", loc),
        format!("ISP Org: {}", org),
    ]
    .into_iter()
    .map(|line| line.len())
    .max()
    .unwrap_or(0) + 4; // Adding padding for borders

    // Create a boundary line
    let boundary = "=".repeat(width);

    // Print the table
    println!("{}", boundary);
    println!("| {:width$} |", format!("IP: {}", ip), width = width - 4);
    println!("| {:width$} |", format!("Country: {}", country), width = width - 4);
    println!("| {:width$} |", format!("Region: {}", region), width = width - 4);
    println!("| {:width$} |", format!("Location: {}", loc), width = width - 4);
    println!("| {:width$} |", format!("ISP Org: {}", org), width = width - 4);
    println!("{}", boundary);
}

// Helper function to extract fields (replace this with actual JSON parsing logic)
fn extract_field(data: &str, key: &str) -> String {
    if let Some(start) = data.find(key) {
        let start_idx = data[start..].find(":").unwrap_or(0) + start + 2;
        let end_idx = data[start_idx..].find(",").unwrap_or(data.len() - start_idx) + start_idx;
        return data[start_idx..end_idx].trim().to_string();
    }
    "N/A".to_string()
}

fn comment_out_other_proxies() {
    // Read the file content
    let content = fs::read_to_string(CONFIG_FILE).expect("Failed to read config file.");
    let supported_protocols = ["socks5", "socks4", "http", "https"];
    let new_content: Vec<String> = content
        .lines()
        .map(|line| {
            if supported_protocols.iter().any(|&protocol| line.trim_start().starts_with(protocol)) {
                format!("# {}", line)  // Comment out the line
            } else {
                line.to_string()  
            }
        })
        .collect();

    // Write the modified content back to the config file
    fs::write(CONFIG_FILE, new_content.join("\n")).expect("Failed to write to config file.");
}

fn add_custom_proxy(custom_proxy: &str) {
    // Open CONFIG_FILE in append mode and ensure that the proxy is added on a new line
    let mut config = OpenOptions::new()
        .append(true)
        .open(CONFIG_FILE)
        .unwrap();

    // Add the proxy to CONFIG_FILE with a newline at the beginning
    writeln!(config, "\n{}", custom_proxy).expect("Failed to add proxy to config.");

    // Open CUSTOM_PROXY_FILE for reading and appending
    let mut custom_file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(CUSTOM_PROXY_FILE)
        .unwrap();

    // Check if the file is empty or if it ends with a newline
    let mut reader = BufReader::new(&custom_file);
    let mut last_line = String::new();
    
    // Read the last line to check if the file ends with a newline
    let is_empty = reader.read_line(&mut last_line).is_err(); // If it's an empty file, this will return error
    if is_empty || last_line.trim().is_empty() {
        // If the file is empty or the last line is empty, add the first proxy
        writeln!(custom_file, "{}", custom_proxy).expect("Failed to add proxy to custom list.");
    } else {
        // If there is content and the last line isn't empty, append the new proxy on a new line
        writeln!(custom_file, "{}", custom_proxy).expect("Failed to add proxy to custom list.");
    }

    println!("[+] Custom proxy added: {}", custom_proxy);
}
fn list_custom_proxies() {
    if let Ok(file) = File::open(CUSTOM_PROXY_FILE) {
        let reader = BufReader::new(file);
        println!("[+] Custom proxies:");
        for (i, line) in reader.lines().enumerate() {
            println!("{}: {}", i + 1, line.unwrap());
        }
    } else {
        println!("[x] No custom proxies found.");
    }
}

fn use_custom_proxy(index: usize){
    // Read CUSTOM_PROXY_FILE so user can choose the coorect proxy 
    let proxes: Vec<String> = if let Ok(file) = File::open(CUSTOM_PROXY_FILE){
        BufReader::new(file).lines().filter_map(Result::ok).collect()
    } else {
        println!("[x] No custom proxies found.");
        return;
    };
    // Change it
    if let Some(proxes) = proxes.get(index - 1){
        enable_proxy(proxes);
        println!("[+] Switched Custom Proxy: {}", proxes);
    }else{
        println!("[x] Invalid selection.");
    }
}

fn delete_custom_proxy(index: usize) {
    // Read CUSTOM_PROXY_FILE so user can choose the coorect proxy 
    let proxies: Vec<String> = if let Ok(file) = File::open(CUSTOM_PROXY_FILE) {
        BufReader::new(file).lines().filter_map(Result::ok).collect()
    } else {
        println!("[x] No custom proxies found.");
        return;
    };

    if let Some(proxy) = proxies.get(index - 1) {
        let remaining: Vec<String> = proxies.iter().filter(|&p| p != proxy).cloned().collect();
        fs::write(CUSTOM_PROXY_FILE, remaining.join("\n")).expect("Failed to update custom list.");

        let config = fs::read_to_string(CONFIG_FILE).expect("Failed to read config file.");
        let new_config = config
            .lines()
            .filter(|line| line != &proxy && !line.trim_start().starts_with(format!("# {}", proxy).as_str()))
            .collect::<Vec<&str>>()
            .join("\n");

        fs::write(CONFIG_FILE, new_config).expect("Failed to update config file.");

        println!("[+] Custom proxy deleted: {}", proxy);
    } else {
        println!("[x] Invalid selection.");
    }
}

fn is_root(){
    let current_uid = get_current_uid();
    if current_uid != 0{
        println!("[x] need root privilege");
        exit(1);
    }
}

fn main() {
    let matches = Command::new("simple Proxychains channger")
        .version("1.0")
        .arg_required_else_help(true)
        .arg(
            Arg::new("tor")
                .long("tor")
                .help("Change proxychains setting to tor network")
                .action(ArgAction::SetTrue),
        ).arg(
            Arg::new("chisel")
                .long("chisel")
                .help("Change proxychains setting to chisel network default: socks5 127.0.0.1 1080")
                .action(ArgAction::SetTrue),
        ).arg(
            Arg::new("add")
                .long("add")
                .help("add your custom proxy address to proxychains config")
                .num_args(1),
        ).arg(
            Arg::new("list")
                .help("list custom proxy u add to proxychains")
                .short('l')
                .action(ArgAction::SetTrue),
        ).arg(
            Arg::new("delete")
                .help("delete proxy in proxychains config")
                .short('d')
                .action(ArgAction::SetTrue),
                // .num_args(1),
        ).arg(
            Arg::new("cs")
                .help("select custom proxy that u add before in proxychains config")
                .long("cs")
                .action(ArgAction::SetTrue),
                // .num_args(1),
        ).arg(
            Arg::new("backup")
                .help("Backup proxychains config to mitigate something bad")
                .short('b')
                .action(ArgAction::SetTrue)
        ).arg(
            Arg::new("restore")
                .help("Restore proxychains config that u backup before")
                .short('r')
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    let tor = matches.get_flag("tor");
    let chisel: bool = matches.get_flag("chisel");
    let cs = matches.get_flag("cs");
    let add = matches.get_one::<String>("add");
    let list = matches.get_flag("list");
    let delete = matches.get_flag("delete");
    let backup = matches.get_flag("backup");
    let restore = matches.get_flag("restore");
    if tor{
        is_root();
        logo();
        comment_out_other_proxies();
        enable_proxy(TOR_PROXY);
        println!("[+] Switched TOR Proxy: {}", TOR_PROXY);
        check();
    }else if chisel{
        is_root();        
        logo();
        comment_out_other_proxies();
        enable_proxy(CHISEL_PROXY);
        println!("[+] Switched Chisel Proxy: {}", CHISEL_PROXY);
        check();
    }else if cs{
        is_root();        
        logo();
        list_custom_proxies();
        println!("Enter the number of the proxy to use:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(index) = input.trim().parse::<usize>() {
            comment_out_other_proxies();
            use_custom_proxy(index);
        }else{
            println!("[x] Invalid input.");
        }
        check();
    }else if let Some(add) = add {
        is_root();        
        logo();
        comment_out_other_proxies();
        add_custom_proxy(add.trim());
        check();
    }else if list{
        logo();
        list_custom_proxies();
    // }else if let Some(delete) = delete {
    }else if delete{
        is_root();
        list_custom_proxies();
        println!("Enter the number of the proxy to delete:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(index) = input.trim().parse::<usize>() {
            delete_custom_proxy(index);
        } else {
            println!("[x] Invalid input.");
        }        
    }else if backup{
        is_root();        
        backupconfig();
    }else if restore{
        is_root();        
        restoreconfig();
    }else{
        println!("[x] Invalid option.");
    }
}
