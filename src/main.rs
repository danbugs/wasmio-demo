use anyhow::Result;

use keyvalue::*;
wit_bindgen_rust::import!("wit/keyvalue_0.3.3/keyvalue.wit");
wit_error_rs::impl_error!(keyvalue::KeyvalueError);

use messaging::*;
wit_bindgen_rust::import!("wit/messaging_0.3.3/messaging.wit");
wit_error_rs::impl_error!(messaging::MessagingError);

fn main() -> Result<()> {
    let my_pub = Pub::open("my-messaging")?;
    let my_sub = Sub::open("my-messaging")?;
    let ping_pong_sub_tok = my_sub.subscribe("ping")?;

    let my_kv = Keyvalue::open("my-container")?;
    println!(">>> Listening...");

    loop {
        let msg = my_sub.receive(&ping_pong_sub_tok)?;

        if !msg.is_empty() {
            let msg_s = String::from_utf8(msg)?;
            match msg_s.as_str() {
                "ping" => {
                    let ret_msg = format!("pong {:?}x", match my_kv.get("increment") {
                        Ok(curr_val) => {
                            let curr_val_s = String::from_utf8(curr_val)?;
                            let increment_count = curr_val_s.as_str().parse::<u32>()? + 1;
                            my_kv.set("increment", increment_count.to_string().as_bytes())?;

                            increment_count
                        }
                        Err(_) => {
                            my_kv.set("increment", "1".as_bytes())?;

                            1
                        }
                    });

                    my_pub.publish(ret_msg.as_bytes(), "pong")?;
                }
                "reset" => {
                    my_kv.set("increment", "0".as_bytes())?;
                    my_pub.publish("pong 0x".as_bytes(), "pong")?;
                }
                _ => {
                    println!("unknown command: {}", msg_s.as_str());
                }
            }
        }
    }
}
