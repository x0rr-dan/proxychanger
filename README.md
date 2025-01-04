# proxychanger
simple tool to auto change proxychains setting

## How this tool work?
is simple just change ucomment proxy that we wanna use in automation tool, im just to lazy to change manualy
```
proxychanger --help 
Usage: proxychanger [OPTIONS]

Options:
      --tor        Change proxychains setting to tor network
      --chisel     Change proxychains setting to chisel network default: socks5 127.0.0.1 1080
      --add <add>  add your custom proxy address to proxychains config
  -l               list custom proxy u add to proxychains
  -d               delete proxy in proxychains config
      --cs         select custom proxy that u add before in proxychains config
  -b               Backup proxychains config to mitigate something bad
  -r               Restore proxychains config that u backup before
  -h, --help       Print help
  -V, --version    Print version
```

```
╰┈➤ proxychanger --tor

 _ __  _ __ _____  ___   _  ___| |__   __ _(_)_ __  ___   
| '_ \| '__/ _ \ \/ / | | |/ __| '_ \ / _` | | '_ \/ __|  
| |_) | | | (_) >  <| |_| | (__| | | | (_| | | | | \__ \  
| .__/|_|  \___/_/\_\__,  |\___|_| |_|\__,_|_|_| |_|___/  
|_|                  |___/                                
              Simple Proxychains Changer
                    coded by x0rr
    
[+] Switched TOR Proxy: socks5 	127.0.0.1 9050
==========================================
| IP: "xx.xxx.xxx.xx"                    |
| Country: "xx"                          |
| Region: "xxxxxxx"                      |
| Location: "xx.xxxx                     |
| ISP Org: "xxxxxxxx xxxx xxxxxxxx xxxx" |
==========================================
```
