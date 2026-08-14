#[cfg(target_os = "android")]
use android_logger::{Config, FilterBuilder};
#[cfg(target_os = "android")]
use log::LevelFilter;
#[cfg(target_os = "android")]
use winit::{
    event_loop::EventLoop,
    platform::android::{EventLoopBuilderExtAndroid, activity::AndroidApp},
};

pub mod app;
pub mod blit;
pub mod dstate;
pub mod render;
pub mod wgpu_state;

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace) // limit log level
            .with_tag("mytag") // logs will show under mytag tag
            .with_filter(FilterBuilder::new().parse("warn,mobile_gfx=trace").build()),
    );

    let event_loop = EventLoop::with_user_event()
        .with_android_app(app)
        .build()
        .unwrap();

    let mut app = app::App::new();
    event_loop.run_app(&mut app).unwrap();
}
