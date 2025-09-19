// src/tool/scan.rs
use owo_colors::OwoColorize;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use std::sync::Arc;
use std::time::Duration;

/// اسکن یک پورت (async)
pub async fn scan_ip_port(ip: &str, port: u16) -> bool {
    let addr = format!("{}:{}", ip, port);
    match timeout(Duration::from_secs(5), TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

pub struct ScanPort {
    pub ip: String,
    pub port: u16,
    pub up: Option<bool>,
}

impl ScanPort {
    pub fn new(ip: &str, port: u16) -> Self {
        Self {
            ip: ip.to_string(),
            port,
            up: None,
        }
    }

    /// متد async که اسکن را اجرا و وضعیت را چاپ می‌کند
    pub async fn scan(&mut self) {
        let result = scan_ip_port(&self.ip, self.port).await;
        self.up = Some(result);

        if result {
            println!(
                "{}",
                format!("{}:{} is open", self.ip, self.port)
                    .green()
                    .bold()
            );
        } else {
            println!(
                "{}",
                format!("{}:{} is closed", self.ip, self.port).red()
            );
        }
    }
}
pub async fn scan_all_port(ip:&str){
    let semaphore = Arc::new(Semaphore::new(1000));
    let mut tasks =Vec::new();
    for port in 1..=65535{
        let ip = ip.to_string();
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        tasks.push(tokio::spawn(async move {
            let is_open  = scan_ip_port(&ip, port).await;
            drop(permit);
            if is_open{
                println!("{}",format!(">> {}:{} is open",ip,port).green().bold());
            }
        }));
    }
    for task in tasks {
        let _ = task.await;
    }
}
pub async fn scanning() {
    use std::io::{self, Write};

    print!("{}", "Enter IP: ".yellow().bold());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let ip = input.trim();

    println!("{}", "Scanning all ports...".blue().bold());
    scan_all_port(ip).await;
    println!("{}", "Scan completed.".blue().bold());
}