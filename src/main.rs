use test_winit_android::run;
use winit::event_loop::EventLoopBuilder;

fn main() {
    tracing_subscriber::fmt().init();
    let event_loop = EventLoopBuilder::new().build().unwrap();
    run(event_loop);
}
