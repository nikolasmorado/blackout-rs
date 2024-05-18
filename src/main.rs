use clap::Parser;
use x11rb::connection::Connection;
use x11rb::protocol::randr::{
    get_crtc_gamma, get_crtc_info, get_screen_resources, set_crtc_gamma, GetCrtcGammaReply,
};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    list: bool,

    #[arg(short, long, value_delimiter = ' ')]
    screens: Vec<u32>,

    #[arg(short, long, default_value = "60")]
    duration: u64,
}

fn blackout(conn: &impl Connection, crtc: u32) -> Result<(), Box<dyn std::error::Error>> {
    let gamma = get_crtc_gamma(conn, crtc)?.reply()?;

    for _ in 0..gamma.red.len() {
        let mut red = gamma.red.clone();
        let mut green = gamma.green.clone();
        let mut blue = gamma.blue.clone();

        for j in 0..red.len() {
            red[j] = 0;
            green[j] = 0;
            blue[j] = 0;
        }

        set_crtc_gamma(conn, crtc, &red, &green, &blue)?;
    }

    Ok(())
}

fn restore(
    conn: &impl Connection,
    crtc: u32,
    gamma: &GetCrtcGammaReply,
) -> Result<(), Box<dyn std::error::Error>> {
    set_crtc_gamma(conn, crtc, &gamma.red, &gamma.green, &gamma.blue)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    if !args.list && args.screens.len() == 0 {
        println!("No command or screens specified, try: blackout --help");

        return Ok(());
    }

    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];

    let sr = get_screen_resources(&conn, screen.root)?.reply()?;

    let mut backup_crtcs = Vec::new();

    for c in sr.crtcs {
        if !args.list && !args.screens.contains(&c) {
            continue;
        }

        let crtc = get_crtc_info(&conn, c, 0)?.reply()?;

        if crtc.width == 0 || crtc.height == 0 {
            continue;
        }

        println!("CRTC {:0?}: {:1?}x {:2?}y", c, crtc.x, crtc.y);
        println!("::: {:0?}x {:1?}y", crtc.width, crtc.height);

        if args.list {
            continue;
        }

        let gamma = get_crtc_gamma(&conn, c)?.reply()?;

        backup_crtcs.push((c, gamma.clone()));

        blackout(&conn, c)?;
    }

    if args.list {
        return Ok(());
    }

    std::thread::sleep(std::time::Duration::from_secs(args.duration));

    for b in backup_crtcs {
        restore(&conn, b.0, &b.1)?;
    }

    let _f = conn.flush();

    Ok(())
}
