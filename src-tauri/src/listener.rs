use std::sync::atomic::AtomicBool;
use std::sync::Mutex;
use std::time::Duration;

use rdev::Event;
use rdev::EventType;
use tauri::Manager;
use tauri::PhysicalPosition;

pub struct Listener {
    right_pressed: AtomicBool,
    start_point: Mutex<Option<(f64, f64)>>,
    end_point: Mutex<(f64, f64)>,
    now_point: Mutex<(f64, f64)>,
    app_handler: Option<tauri::AppHandle>,
}

impl Listener {
    pub fn new() -> Self {
        Self {
            right_pressed: AtomicBool::new(false),
            start_point: Mutex::new(None),
            end_point: Mutex::new((0.0, 0.0)),
            now_point: Mutex::new((0.0, 0.0)),
            app_handler: None,
        }
    }

    pub fn init_app_handler(&mut self, app_handler: tauri::AppHandle) {
        self.app_handler = Some(app_handler.clone());
    }

    pub fn listen(mut self) {
        rdev::listen(move |event: Event| self.callback(event)).unwrap();
    }

    fn callback(&mut self, event: Event) {
        let auto_hide = || {
            if !self.app_handler.is_some() {
                return;
            }
            let handle = self.app_handler.clone().unwrap();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(100));
                let main_window = handle.get_window("main").unwrap();
                if !main_window.is_focused().unwrap() {
                    main_window.hide().unwrap();
                }
            });
        };
        match event.event_type {
            EventType::KeyPress(key) => {
                println!("Key press: {:?}", key);
            }
            EventType::ButtonPress(button) => {
                auto_hide();
                match button {
                    rdev::Button::Right => {
                        self.right_pressed
                            .store(true, std::sync::atomic::Ordering::SeqCst);
                        println!("Pressed");
                    }
                    rdev::Button::Middle => {
                        if self.app_handler.is_some() {
                            let handler = self.app_handler.as_ref().unwrap();
                            let main_window = handler.get_window("main").unwrap();
                            main_window.set_always_on_top(true).unwrap();
                            self.now_point
                                .lock()
                                .and_then(|now_point| {
                                    let siz = main_window.outer_size().unwrap();
                                    let x = now_point.0 - siz.width as f64 / 2.0;
                                    let y = now_point.1 - siz.height as f64 / 2.0;
                                    main_window.set_position(PhysicalPosition { x, y }).unwrap();
                                    Ok(())
                                })
                                .unwrap();
                            main_window.show().unwrap();
                            main_window.set_focus().unwrap();
                        }
                    }
                    _ => (),
                }
            }
            EventType::ButtonRelease(button) => match button {
                rdev::Button::Right => {
                    self.right_pressed
                        .store(false, std::sync::atomic::Ordering::SeqCst);
                    self.start_point
                        .lock()
                        .and_then(|mut start_point| {
                            Ok(self
                                .end_point
                                .lock()
                                .and_then(|end_point| {
                                    println!("Start: {:?}, End: {:?}", *start_point, *end_point);
                                    *start_point = None;
                                    Ok(())
                                })
                                .unwrap())
                        })
                        .unwrap();
                }
                _ => (),
            },
            EventType::MouseMove { x, y } => {
                let pressed = self.right_pressed.load(std::sync::atomic::Ordering::SeqCst);
                if pressed {
                    self.start_point
                        .lock()
                        .and_then(|mut start_point| {
                            if *start_point == None {
                                *start_point = Some((x, y));
                            }
                            Ok(())
                        })
                        .unwrap();
                    self.end_point
                        .lock()
                        .and_then(|mut end_point| {
                            *end_point = (x, y);
                            Ok(())
                        })
                        .unwrap();
                }
                self.now_point
                    .lock()
                    .and_then(|mut now_point| {
                        *now_point = (x, y);
                        Ok(())
                    })
                    .unwrap();
            }
            _ => (),
        }
    }
}
