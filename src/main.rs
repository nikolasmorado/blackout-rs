use x11rb::connection::Connection;
use x11rb::protocol::randr::{get_crtc_gamma, get_crtc_info, get_screen_resources, set_crtc_gamma};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];

    let sr = get_screen_resources(&conn, screen.root)?.reply()?;

    let mut backup_crtcs = Vec::new();

    for c in sr.crtcs {
        let crtc = get_crtc_info(&conn, c, 0)?.reply()?;

        if crtc.width == 0 || crtc.height == 0 {
            continue;
        }

        println!("CRTC {:0?}: {:1?}x {:2?}y", c, crtc.x, crtc.y);
        println!("::: {:0?}x {:1?}y", crtc.width, crtc.height);

        let gamma = get_crtc_gamma(&conn, c)?.reply()?;

        backup_crtcs.push((c, gamma.clone()));

        for _ in 0..gamma.red.len() {
            let mut red = gamma.red.clone();
            let mut green = gamma.green.clone();
            let mut blue = gamma.blue.clone();

            for j in 0..red.len() {
                red[j] = 0;
                green[j] = 0;
                blue[j] = 0;
            }

            set_crtc_gamma(&conn, c, &red, &green, &blue)?;
        }
    }
    
    std::thread::sleep(std::time::Duration::from_secs(2));

    for b in backup_crtcs {
        println!("Setting gamma for CRTC: {:?}", b.0);
        set_crtc_gamma(&conn, b.0, &b.1.red, &b.1.green, &b.1.blue)?;
    }

    let _f = conn.flush();

    loop {
        println!("Event: {:?}", conn.wait_for_event()?);
    }
}
