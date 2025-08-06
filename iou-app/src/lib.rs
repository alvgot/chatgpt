
use smol_iroh::{start_node, send_credit, Credit};


/// Android entry point using ndk-glue's main macro.
/// This spawns a local node and sends a demonstration credit.
#[ndk_glue::main]
fn android_main(_app: ndk_glue::App) {
    smol::block_on(async {
        smol::spawn(start_node("127.0.0.1:7000")).detach();
        smol::Timer::after(std::time::Duration::from_millis(100)).await;
        let credit = Credit::new("alice", "bob", 5);
        let path = vec!["127.0.0.1:7000".to_string()];
        let _ = route_credit(&path, credit).await;
    });
}
