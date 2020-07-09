use crate::widgets::smiley::Smiley;
use cursor::Cursor;
use piet_web::WebRenderContext;
use serde::{Deserialize, Serialize};
use std::f64;

pub mod cursor;
pub mod rect;
pub mod smiley;

// --- Trait ---
pub(crate) trait Draw<T> {
    fn update(&mut self, ui_state: &mut UiGlobalState) -> T;
    fn change(&mut self, changes: &RectGeometry);
    fn draw(&self, context: &mut WebRenderContext, ui_state: &UiGlobalState);
}

// --- Geometry ---
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct RectGeometry {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// --- Widget State ---
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WidgetState {
    pub uuid: String,
    pub selected: bool,
    pub hovered: bool,
    pub geometry: RectGeometry,
}

// --- UI State ---
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UiGlobalState {
    pub cursor: Cursor, // Mouse Cursor on Canvas
    pub canvas_geometry: RectGeometry,
    pub active_widget_uuid: Option<String>,
    pub pointer_widget_uuid: Option<String>, // from mouse, during draw loop
}

// --- Other ---
#[derive(Default, Serialize, Deserialize)]
pub struct FrontEnd {
    widgets: Vec<Smiley>,
    ui_state: UiGlobalState,
}
