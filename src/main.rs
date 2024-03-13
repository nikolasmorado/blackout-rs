use x11rb::connection::Connection;
use x11rb::protocol::randr;
use tokio::task;
use futures::future::join_all;

const SLEEP_TIME: u64 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, _screen_num) = x11rb::connect(None).unwrap();

    println!();
    println!("Connected to server ");

    conn.setup().roots.iter().for_each(|screen| {
        println!("Screen: {}, {}x{}", screen.root, screen.width_in_pixels, screen.height_in_pixels);

        let resources: randr::GetScreenResourcesReply =
            randr::get_screen_resources(&conn, screen.root).unwrap()
                .reply().unwrap();
        
        resources.crtcs.iter().for_each(|crtc| {
            println!("Found crtc {}", crtc);
            let gamma: randr::GetCrtcGammaReply = randr::get_crtc_gamma(&conn, *crtc).unwrap().reply().unwrap();
            let gamma_size = gamma.red.len();
            let black_gamma = vec![0; gamma_size];
            let _set_result = randr::set_crtc_gamma(&conn, *crtc, &black_gamma, &black_gamma, &black_gamma);
            while let Ok(updated_gamma) = randr::get_crtc_gamma(&conn, *crtc) {
                let get_reply: randr::GetCrtcGammaReply = updated_gamma.reply().unwrap();
                if get_reply.red == black_gamma
                    && get_reply.green == black_gamma
                    && get_reply.blue == black_gamma
                {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(SLEEP_TIME));
            let _set_result = randr::set_crtc_gamma(&conn, *crtc, &gamma.red, &gamma.green, &gamma.blue);
        });

    });


    println!();

    drop(conn);

    Ok(())
}
