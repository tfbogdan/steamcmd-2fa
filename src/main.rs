use clap::Parser;
use simple_steam_totp::{generate};

fn find_default_steamcmd() -> &'static str {
    if cfg!(target_os = "windows") {
        "C:\\steamcmd\\steamcmd.exe"
    } else {
        if (std::path::Path::new("/home/steam/steamcmd/steamcmd.sh")).exists() {
            "/home/steam/steamcmd/steamcmd.sh"
        } else {
            "/home/steam/steamcmd"
        }
    }
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    // Steam username
    #[clap(short, long)]
    username: String,

    // Steam password
    #[clap(short, long)]
    password: String,

    // Steam 2FA shared secret
    #[clap(short, long)]
    secret: String
}

fn main() {
    let args = Args::parse();

    let totp = match generate(&args.secret) {
        Ok(code) => code,
        Err(e) => {
            println!("Failed to generate Steam TOTP code: {}", e);
            std::process::exit(1);
        }
    };

    println!("{}", &totp);

    std::process::exit(0);
}