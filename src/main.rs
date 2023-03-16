use anyhow::Result;

use keyvalue::*;
wit_bindgen_rust::import!("wit/keyvalue_0.3.3/keyvalue.wit");
wit_error_rs::impl_error!(keyvalue::KeyvalueError);

use messaging::*;
wit_bindgen_rust::import!("wit/messaging_0.3.3/messaging.wit");
wit_error_rs::impl_error!(messaging::MessagingError);

fn main() -> Result<()> {
    let my_sub = Sub::open("my-messaging")?;
    let ping_pong_sub_tok = my_sub.subscribe("ping-pong")?;

    let my_kv = Keyvalue::open("my-container")?;

    loop {
        let msg = my_sub.receive(&ping_pong_sub_tok)?;

        if !msg.is_empty() {
            let msg_s = String::from_utf8(msg)?;
            match msg_s.as_str() {
                "ping" => match my_kv.get("increment") {
                    Ok(curr_val) => {
                        let curr_val_s = String::from_utf8(curr_val)?;
                        let increment_count = curr_val_s.as_str().parse::<u32>()? + 1;
                        my_kv.set("increment", increment_count.to_string().as_bytes())?;
                    }
                    Err(_) => {
                        my_kv.set("increment", "1".as_bytes())?;
                    }
                },
                "pong" => match my_kv.get("increment") {
                    Ok(curr_val) => {
                        println!("pong: {}", String::from_utf8(curr_val)?);
                    }
                    Err(_) => {
                        println!("pong: 0");
                        my_kv.set("increment", "0".as_bytes())?;
                    }
                },
                "reset" => {
                    my_kv.set("increment", "0".as_bytes())?;
                }
                _ => {
                    println!("unknown command: {}", msg_s.as_str());
                }
            }
        }
    }
}
