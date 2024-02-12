use x11rb::connection::Connection;
use x11rb::protocol::randr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();

    print!("Connected to server ");
    let screen = &conn.setup().roots[screen_num];

    let resources: randr::GetScreenResourcesReply = randr::get_screen_resources(&conn, screen.root)?.reply()?;
    
    let crtcs = resources.crtcs.clone();

    for crtc in crtcs {
        let gamma: randr::GetCrtcGammaReply = randr::get_crtc_gamma(&conn, crtc)?.reply()?;

        let red_black = vec![0; gamma.red.len() as usize];
        let green_black = vec![0; gamma.green.len() as usize];
        let blue_black = vec![0; gamma.blue.len() as usize];

        let set_result: randr::GetCrtcGammaReply = randr::set_crtc_gamma(&conn, crtc, &red_black, &green_black, &blue_black);

        

        // while let Ok(updated_gamma) = randr::get_crtc_gamma(&conn, crtc) {
        //     let get_reply: randr::GetCrtcGammaReply = updated_gamma.reply()?;
        //     if get_reply.red == red_black && get_reply.green == green_black && get_reply.blue == blue_black {
        //         break;
        //     }
        // }

        std::thread::sleep(std::time::Duration::from_secs(3));


        let _ = randr::set_crtc_gamma(&conn, crtc, &gamma.red, &gamma.green, &gamma.blue);

    }

    drop(conn);

    Ok(())
}
 
