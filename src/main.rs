use tokio::select;

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
    #[cfg(target_family = "unix")]
    {
        let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
            .expect("信号创建失败");
        // let mut sighup = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::hangup())
        //     .expect("信号创建失败");
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("信号创建失败");

        select! {
            _ = _wow.run() => {},
            _ = sigint.recv() => {
                _wow._stop();
            },
            _ = sigterm.recv() => {
                _wow._stop();
            },
            // 不监听挂起信号
            // 使得终端关闭后进程仍能运行
            // _ = sighup.recv() => {
            //     _wow._stop();
            // },
        }
    }
    #[cfg(target_family = "windows")]
    {
        select! {
            _ = _wow.run() => {},
            _ = tokio::signal::ctrl_c() => {
                _wow._stop();
            },
        }
    }
}
