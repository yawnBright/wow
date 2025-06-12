mod config;
mod wow;

// WOW文件布局
// bin/wow
// image/today.jpg&png
// record.log
// setup.sh
//

#[tokio::main]
async fn main() {
    let mut _wow = wow::Wow::new();
    _wow.run().await;
}
