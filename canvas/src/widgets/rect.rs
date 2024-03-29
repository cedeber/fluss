use super::RectGeometry;
use crate::{
    widgets::{cursor::POINTER_SIZE, Draw, UiGlobalState},
    BLUE, PINK, WHITE,
};
use nalgebra::geometry::Point2;
use ncollide2d::bounding_volume::{BoundingSphere, BoundingVolume};
use piet::{
    kurbo::{BezPath, Circle, Line, Rect as Rectangle},
    RenderContext, StrokeStyle,
};
use serde::{Deserialize, Serialize};

static RECT_ANCHOR_RADIUS: f64 = 3.0;

pub enum RectState {
    Hover,
    Select,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Anchor {
    None,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Default)]
struct Properties {
    border_left: f64,
    border_top: f64,
    border_right: f64,
    border_bottom: f64,
    line_left: f64,
    line_top: f64,
    line_right: f64,
    line_bottom: f64,
    radius: f64,
}

pub struct Rect {
    pub geometry: RectGeometry,
    pub state: RectState,
    active_anchor: Anchor,
    properties: Properties,
}

impl Rect {
    pub fn new(geometry: RectGeometry, state: RectState) -> Self {
        Self {
            geometry,
            state,
            active_anchor: Anchor::None,
            properties: get_properties(&geometry),
        }
    }
}

impl Draw<RectGeometry> for Rect {
    fn update(&mut self, ui_state: &mut UiGlobalState) -> RectGeometry {
        match self.state {
            RectState::Hover => RectGeometry::default(),
            RectState::Select => self.update_select(ui_state),
        }
    }

    fn change(&mut self, _changes: &RectGeometry) {
        unimplemented!();
    }

    fn draw(&self, context: &mut impl RenderContext, ui_state: &UiGlobalState) {
        match self.state {
            RectState::Hover => self.draw_hover(context, ui_state),
            RectState::Select => self.draw_select(context, ui_state),
        }
    }
}

impl Rect {
    // Instead of change()
    pub fn reset_geometry(&mut self, geometry: &RectGeometry) {
        self.geometry = *geometry;
        self.properties = get_properties(&geometry);
    }

    fn draw_hover(&self, context: &mut impl RenderContext, _ui_state: &UiGlobalState) {
        // let viewport_width = ui_state.canvas_geometry.width;
        // let viewport_height = ui_state.canvas_geometry.width;

        // Rectangle
        let brush = context.solid_brush(BLUE);
        context.stroke(
            Rectangle::new(
                self.properties.border_left - 0.5,
                self.properties.border_top - 0.5,
                self.properties.border_right + 0.5,
                self.properties.border_bottom + 0.5,
            ),
            &brush,
            2.,
        );
    }

    /// Rect with Anchors -> Change Widget Geometry
    fn update_select(&mut self, ui_state: &mut UiGlobalState) -> RectGeometry {
        let mut geometry_state = RectGeometry::default();

        // Anchors
        if let Some(cursor_position) = &ui_state.cursor.position {
            let cursor_bounding = BoundingSphere::new(*cursor_position, POINTER_SIZE);
            let top_left_anchor_bounding = BoundingSphere::new(
                Point2::new(self.properties.border_left, self.properties.border_top),
                self.properties.radius,
            );
            let top_right_anchor_bounding = BoundingSphere::new(
                Point2::new(self.properties.border_right, self.properties.border_top),
                self.properties.radius,
            );
            let bottom_left_anchor_bounding = BoundingSphere::new(
                Point2::new(self.properties.border_left, self.properties.border_bottom),
                self.properties.radius,
            );
            let bottom_right_anchor_bounding = BoundingSphere::new(
                Point2::new(self.properties.border_right, self.properties.border_bottom),
                self.properties.radius,
            );

            self.active_anchor = if cursor_bounding.intersects(&top_left_anchor_bounding) {
                Anchor::TopLeft
            } else if cursor_bounding.intersects(&top_right_anchor_bounding) {
                Anchor::TopRight
            } else if cursor_bounding.intersects(&bottom_left_anchor_bounding) {
                Anchor::BottomLeft
            } else if cursor_bounding.intersects(&bottom_right_anchor_bounding) {
                Anchor::BottomRight
            } else {
                Anchor::None
            };
        }

        let anchor_is_active = self.active_anchor != Anchor::None;

        if ui_state.cursor.down_start_position.is_some() {
            if ui_state.cursor.is_active || anchor_is_active {
                if anchor_is_active && ui_state.cursor.active_anchor == Anchor::None {
                    ui_state.cursor.active_anchor = self.active_anchor;
                }

                if let Some(cursor_position) = &ui_state.cursor.position {
                    if let Some(cursor_previous_position) = &ui_state.cursor.previous_position {
                        let x = cursor_position.x - cursor_previous_position.x;
                        let y = cursor_position.y - cursor_previous_position.y;
                        let keep_ration = ui_state.settings.keep_ratio;

                        match ui_state.cursor.active_anchor {
                            Anchor::TopLeft => {
                                geometry_state.x = x;
                                geometry_state.y = if keep_ration { x } else { y };
                                geometry_state.width = -x;
                                geometry_state.height = if keep_ration { -x } else { -y };
                            }
                            Anchor::TopRight => {
                                geometry_state.y = if keep_ration { -x } else { y };
                                geometry_state.width = x;
                                geometry_state.height = if keep_ration { x } else { -y };
                            }
                            Anchor::BottomRight => {
                                geometry_state.width = x;
                                geometry_state.height = if keep_ration { x } else { y };
                            }
                            Anchor::BottomLeft => {
                                geometry_state.x = x;
                                geometry_state.width = -x;
                                geometry_state.height = if keep_ration { -x } else { y };
                            }
                            _ => {}
                        }
                    }
                }
                ui_state.cursor.is_active = true;
            }
        } else {
            // reset - pointer out
            ui_state.cursor.active_anchor = Anchor::None;
            // keep active in case it is over an anchor but not pointerdown
            ui_state.cursor.is_active = anchor_is_active;
        }

        geometry_state
    }

    fn draw_select(&self, context: &mut impl RenderContext, ui_state: &UiGlobalState) {
        let viewport_width = ui_state.canvas_geometry.width;
        let viewport_height = ui_state.canvas_geometry.width;
        // ui_state active anchor has priority
        let active_anchor = if ui_state.cursor.active_anchor != Anchor::None {
            ui_state.cursor.active_anchor
        } else {
            self.active_anchor
        };

        // -> Rectangle
        let brush = context.solid_brush(PINK);
        context.stroke(
            Rectangle::new(
                self.properties.border_left,
                self.properties.border_top,
                self.properties.border_right,
                self.properties.border_bottom,
            ),
            &brush,
            1.0,
        );

        // -> Anchors
        let fill_brush = context.solid_brush(WHITE);
        let fill_brush_active = context.solid_brush(PINK);

        if ui_state.settings.keep_ratio {
            let mut path = BezPath::new();
            path.move_to((
                self.properties.border_left + 3.,
                self.properties.border_top - 7.,
            ));
            path.quad_to(
                (
                    self.properties.border_left - 8.5,
                    self.properties.border_top - 8.5,
                ),
                (
                    self.properties.border_left - 7.,
                    self.properties.border_top + 3.,
                ),
            );
            context.stroke(path, &fill_brush_active, 2.0);
        }

        context.fill(
            Circle::new(
                (self.properties.border_left, self.properties.border_top),
                self.properties.radius,
            ),
            if active_anchor == Anchor::TopLeft {
                &fill_brush_active
            } else {
                &fill_brush
            },
        );
        context.stroke(
            Circle::new(
                (self.properties.border_left, self.properties.border_top),
                self.properties.radius,
            ),
            &brush,
            2.,
        );

        if ui_state.settings.keep_ratio {
            let mut path = BezPath::new();
            path.move_to((
                self.properties.border_right - 3.,
                self.properties.border_top - 7.,
            ));
            path.quad_to(
                (
                    self.properties.border_right + 8.5,
                    self.properties.border_top - 8.5,
                ),
                (
                    self.properties.border_right + 7.,
                    self.properties.border_top + 3.,
                ),
            );
            context.stroke(path, &fill_brush_active, 2.0);
        }

        context.fill(
            Circle::new(
                (self.properties.border_right, self.properties.border_top),
                self.properties.radius,
            ),
            if active_anchor == Anchor::TopRight {
                &fill_brush_active
            } else {
                &fill_brush
            },
        );
        context.stroke(
            Circle::new(
                (self.properties.border_right, self.properties.border_top),
                self.properties.radius,
            ),
            &brush,
            2.,
        );

        if ui_state.settings.keep_ratio {
            let mut path = BezPath::new();
            path.move_to((
                self.properties.border_right - 3.,
                self.properties.border_bottom + 7.,
            ));
            path.quad_to(
                (
                    self.properties.border_right + 8.5,
                    self.properties.border_bottom + 8.5,
                ),
                (
                    self.properties.border_right + 7.,
                    self.properties.border_bottom - 3.,
                ),
            );
            context.stroke(path, &fill_brush_active, 2.0);
        }

        context.fill(
            Circle::new(
                (self.properties.border_right, self.properties.border_bottom),
                self.properties.radius,
            ),
            if active_anchor == Anchor::BottomRight {
                &fill_brush_active
            } else {
                &fill_brush
            },
        );
        context.stroke(
            Circle::new(
                (self.properties.border_right, self.properties.border_bottom),
                self.properties.radius,
            ),
            &brush,
            2.,
        );

        if ui_state.settings.keep_ratio {
            let mut path = BezPath::new();
            path.move_to((
                self.properties.border_left + 3.,
                self.properties.border_bottom + 7.,
            ));
            path.quad_to(
                (
                    self.properties.border_left - 8.5,
                    self.properties.border_bottom + 8.5,
                ),
                (
                    self.properties.border_left - 7.,
                    self.properties.border_bottom - 3.,
                ),
            );
            context.stroke(path, &fill_brush_active, 2.0);
        }

        context.fill(
            Circle::new(
                (self.properties.border_left, self.properties.border_bottom),
                self.properties.radius,
            ),
            if active_anchor == Anchor::BottomLeft {
                &fill_brush_active
            } else {
                &fill_brush
            },
        );
        context.stroke(
            Circle::new(
                (self.properties.border_left, self.properties.border_bottom),
                self.properties.radius,
            ),
            &brush,
            2.,
        );

        // -> Stroked lines
        let mut line_stroke = StrokeStyle::new();
        line_stroke.set_dash_pattern(vec![5.0, 3.0]);
        let brush = context.solid_brush(PINK);

        // Top
        context.stroke_styled(
            Line::new(
                (0.0, self.properties.line_top),
                (
                    self.properties.line_left - RECT_ANCHOR_RADIUS * 2.,
                    self.properties.line_top,
                ),
            ),
            &brush,
            1.,
            &line_stroke,
        );

        context.stroke_styled(
            Line::new(
                (viewport_width, self.properties.line_top),
                (
                    self.properties.line_right + RECT_ANCHOR_RADIUS * 2.,
                    self.properties.line_top,
                ),
            ),
            &brush,
            1.,
            &line_stroke,
        );

        // Bottom
        context.stroke_styled(
            Line::new(
                (0.0, self.properties.line_bottom),
                (
                    self.properties.line_left - RECT_ANCHOR_RADIUS * 2.,
                    self.properties.line_bottom,
                ),
            ),
            &brush,
            1.,
            &line_stroke,
        );

        context.stroke_styled(
            Line::new(
                (viewport_width, self.properties.line_bottom),
                (
                    self.properties.line_right + RECT_ANCHOR_RADIUS * 2.,
                    self.properties.line_bottom,
                ),
            ),
            &brush,
            1.,
            &line_stroke,
        );

        // Left
        context.stroke_styled(
            Line::new(
                (self.properties.line_left, 0.0),
                (
                    self.properties.line_left,
                    self.properties.border_top - RECT_ANCHOR_RADIUS * 2.,
                ),
            ),
            &brush,
            1.,
            &line_stroke,
        );

        context.stroke_styled(
            Line::new(
                (self.properties.line_left, viewport_height),
                (
                    self.properties.line_left,
                    self.properties.border_bottom + RECT_ANCHOR_RADIUS * 2.,
                ),
            ),
            &brush,
            1.,
            &line_stroke,
        );

        // Right
        context.stroke_styled(
            Line::new(
                (self.properties.line_right, 0.0),
                (
                    self.properties.line_right,
                    self.properties.border_top - RECT_ANCHOR_RADIUS * 2.,
                ),
            ),
            &brush,
            1.,
            &line_stroke,
        );

        context.stroke_styled(
            Line::new(
                (self.properties.line_right, viewport_height),
                (
                    self.properties.line_right,
                    self.properties.border_bottom + RECT_ANCHOR_RADIUS * 2.,
                ),
            ),
            &brush,
            1.,
            &line_stroke,
        );
    }
}

fn get_properties(geometry: &RectGeometry) -> Properties {
    let border_width = geometry.width; // + RECT_BORDER_WIDTH;
    let border_height = geometry.height; // + RECT_BORDER_WIDTH;
    let border_left = geometry.x + 0.5; // - RECT_BORDER_WIDTH / 2.;
    let border_top = geometry.y + 0.5; // - RECT_BORDER_WIDTH / 2.;
    let border_right = border_left + border_width;
    let border_bottom = border_top + border_height;
    let line_left = geometry.x + 0.5;
    let line_top = geometry.y + 0.5;
    let line_right = line_left + geometry.width;
    let line_bottom = line_top + geometry.height;

    Properties {
        border_left,
        border_top,
        border_right,
        border_bottom,
        line_left,
        line_top,
        line_right,
        line_bottom,
        radius: RECT_ANCHOR_RADIUS + 0.5,
    }
}
