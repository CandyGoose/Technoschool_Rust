use clap::{Arg, Command};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() -> io::Result<()> {
    let matches = Command::new("telnet")
        .version("1.0")
        .about("Simple Telnet client")
        .arg(Arg::new("host")
            .help("The host to connect to")
            .required(true)
            .index(1))
        .arg(Arg::new("port")
            .help("The port to connect to")
            .required(true)
            .index(2))
        .arg(Arg::new("timeout")
            .long("timeout")
            .help("Connection timeout duration (default is 10s)")
            .default_value("10s"))
        .get_matches();

    let host = matches.get_one::<String>("host").unwrap();
    let port = matches.get_one::<String>("port").unwrap();

    let timeout_str = matches.get_one::<String>("timeout").unwrap();
    let timeout_duration = parse_timeout(timeout_str).unwrap_or(Duration::from_secs(10));

    let address = format!("{}:{}", host, port);
    let socket_addrs = address.to_socket_addrs()?.collect::<Vec<_>>();
    if socket_addrs.is_empty() {
        eprintln!("Invalid host or port: {}", address);
        return Ok(());
    }

    let stream = match TcpStream::connect_timeout(&socket_addrs[0], timeout_duration) {
        Ok(stream) => {
            println!("Connected to {}", address);
            stream
        }
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", address, e);
            return Ok(());
        }
    };

    stream.set_read_timeout(Some(timeout_duration))?;
    stream.set_write_timeout(Some(timeout_duration))?;

    let stream_clone = stream.try_clone()?;
    let mut reader = BufReader::new(stream_clone);

    let handle = std::thread::spawn(move || {
        let mut buffer = String::new();
        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(0) => {
                    println!("Connection closed by server.");
                    break;
                }
                Ok(_) => {
                    print!("{}", buffer);
                    io::stdout().flush().unwrap();
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }
        }
    });

    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut stream_writer = &stream;

    loop {
        input.clear();
        print!("> ");
        stdout.flush()?;

        match stdin.read_line(&mut input) {
            Ok(0) => {
                println!("EOF received, closing connection.");
                break;
            }
            Ok(_) => {
                // Отправляем данные в сокет
                if let Err(e) = stream_writer.write_all(input.as_bytes()) {
                    eprintln!("Error sending data to server: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break;
            }
        }
    }

    drop(stream);

    handle.join().unwrap();

    Ok(())
}

fn parse_timeout(timeout_str: &str) -> Option<Duration> {
    if let Some(seconds) = timeout_str.strip_suffix("s") {
        if let Ok(secs) = seconds.parse::<u64>() {
            return Some(Duration::from_secs(secs));
        }
    }
    if let Some(minutes) = timeout_str.strip_suffix("m") {
        if let Ok(mins) = minutes.parse::<u64>() {
            return Some(Duration::from_secs(mins * 60));
        }
    }
    None
}
