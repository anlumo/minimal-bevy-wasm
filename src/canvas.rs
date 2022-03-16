use bevy::{prelude::*, window::WindowId};
use raw_window_handle::{RawWindowHandle, WebHandle};
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

#[derive(Default)]
pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        let window_descriptor = app
            .world
            .get_resource::<WindowDescriptor>()
            .unwrap()
            .clone();
        let mut windows = app.world.get_resource_mut::<Windows>().unwrap();
        let mut handle = WebHandle::empty();
        if let Some(raw_handle) = &window_descriptor.canvas {
            let id = raw_handle.parse().unwrap_or_default();
            handle.id = id;
            let canvas: HtmlCanvasElement = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .query_selector(&format!("[data-raw-handle=\"{id}\"]"))
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            canvas.set_width(window_descriptor.width as _);
            canvas.set_height(window_descriptor.height as _);
        }
        let width = window_descriptor.width as _;
        let height = window_descriptor.height as _;
        windows.add(Window::new(
            WindowId::primary(),
            &window_descriptor,
            width,
            height,
            1.0,
            None,
            RawWindowHandle::Web(handle),
        ));
    }
}
