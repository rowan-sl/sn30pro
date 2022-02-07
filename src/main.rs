use sn30pro::Controller;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let js_id = 0;
    let mut controller = Controller::new(js_id).await?;
    loop {
        if controller.update().await? {
            println!("{:#?}", controller);
        }
    }
}
