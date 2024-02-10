
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder
};


pub fn run(event_loop: EventLoop<()>) {
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop
        .run(move |event, event_loop| match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput { event: KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },..} => event_loop.exit(),
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}


fn _main(event_loop: EventLoop<()>) {
    run(event_loop);
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    
    use winit::event_loop::EventLoopBuilder;
    use winit::platform::android::EventLoopBuilderExtAndroid;

    log::trace!("Testfjiozehgoizh");

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Trace)
            .with_tag("mytag"),
    );

    /*let mut quit = false;


    while !quit {
        app.poll_events(None, |event| match event {
            PollEvent::Wake => log::trace!("Wake up"),
            PollEvent::Timeout => log::trace!("Timed out"),
            PollEvent::Main(main_event) => match main_event {
                MainEvent::Destroy => quit = true,
                _ => {}
            },
            _ => {}
        });
    }*/

    let event_loop = EventLoopBuilder::new().with_android_app(app).build().unwrap();
    _main(event_loop);
}
