use super::RectGeometry;
use crate::{
    widgets::{Draw, GeometryChangeState, UiGlobalState},
    BLUE, PINK, POINTER_SIZE, RECT_ANCHOR_RADIUS, RECT_BORDER_WIDTH, WHITE,
};
use nalgebra::geometry::Point2;
use ncollide2d::bounding_volume::{BoundingSphere, BoundingVolume};
use piet::{
    kurbo::{Circle, Line, Rect as Rectangle},
    Color, RenderContext, StrokeStyle,
};
use piet_web::WebRenderContext;

pub enum RectState {
    Hover,
    Select,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Anchor {
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

impl Draw<GeometryChangeState> for Rect {
    fn update(&mut self, ui_state: &mut UiGlobalState) -> GeometryChangeState {
        match self.state {
            RectState::Hover => GeometryChangeState::default(),
            RectState::Select => self.update_select(ui_state),
        }
    }

    fn change(&mut self, changes: GeometryChangeState) -> GeometryChangeState {
        self.geometry = changes.geometry;
        self.properties = get_properties(&changes.geometry);

        GeometryChangeState::default()
    }

    fn draw(&self, context: &mut WebRenderContext, ui_state: &UiGlobalState) {
        match self.state {
            RectState::Hover => self.draw_hover(context, ui_state),
            RectState::Select => self.draw_select(context, ui_state),
        }
    }
}

impl Rect {
    fn draw_hover(&self, context: &mut WebRenderContext, ui_state: &UiGlobalState) {
        let viewport_width = ui_state.canvas_geometry.width;
        let viewport_height = ui_state.canvas_geometry.width;

        // Rectangle
        let brush = context.solid_brush(BLUE);
        context.stroke(
            Rectangle::new(
                self.properties.border_left,
                self.properties.border_top,
                self.properties.border_right,
                self.properties.border_bottom,
            ),
            &brush,
            RECT_BORDER_WIDTH,
        );

        // Stroked lines
        // let mut line_stroke = StrokeStyle::new();
        // line_stroke.set_dash(vec![5.0, 3.0], 0.0);

        // let brush = context.solid_brush(BLUE);

        // context.stroke_styled(
        //     Line::new(
        //         (self.properties.line_left, 0.0),
        //         (self.properties.line_left, viewport_height),
        //     ),
        //     &brush,
        //     1.,
        //     &line_stroke,
        // );

        // context.stroke_styled(
        //     Line::new(
        //         (0.0, self.properties.line_top),
        //         (viewport_width, self.properties.line_top),
        //     ),
        //     &brush,
        //     1.,
        //     &line_stroke,
        // );

        // context.stroke_styled(
        //     Line::new(
        //         (self.properties.line_right, 0.0),
        //         (self.properties.line_right, viewport_height),
        //     ),
        //     &brush,
        //     1.,
        //     &line_stroke,
        // );

        // context.stroke_styled(
        //     Line::new(
        //         (0.0, self.properties.line_bottom),
        //         (viewport_width, self.properties.line_bottom),
        //     ),
        //     &brush,
        //     1.,
        //     &line_stroke,
        // );
    }

    fn update_select(&mut self, ui_state: &mut UiGlobalState) -> GeometryChangeState {
        let mut geometry_state = GeometryChangeState::default();

        // Anchors
        if let Some(cursor_position) = &ui_state.cursor.position {
            let cursor_bounding = BoundingSphere::new(cursor_position.clone(), POINTER_SIZE);
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

        let cursor_is_active = self.active_anchor != Anchor::None;

        if let Some(_pos) = &ui_state.cursor.down_start_position {
            if ui_state.cursor.is_active || cursor_is_active {
                if let Some(cursor_position) = &ui_state.cursor.position {
                    if let Some(cursor_previous_position) = &ui_state.cursor.previous_position {
                        geometry_state.geometry.x = cursor_position.x - cursor_previous_position.x;
                        geometry_state.geometry.y = cursor_position.y - cursor_previous_position.y;
                    }
                }
                ui_state.cursor.is_active = true;
            }
        } else {
            ui_state.cursor.is_active = cursor_is_active;
        }

        geometry_state
    }

    fn draw_select(&self, context: &mut WebRenderContext, ui_state: &UiGlobalState) {
        let viewport_width = ui_state.canvas_geometry.width;
        let viewport_height = ui_state.canvas_geometry.width;
        let active_anchor = self.active_anchor;

        // Rectangle
        let brush = context.solid_brush(PINK);
        context.stroke(
            Rectangle::new(
                self.properties.border_left,
                self.properties.border_top,
                self.properties.border_right,
                self.properties.border_bottom,
            ),
            &brush,
            RECT_BORDER_WIDTH,
        );

        // Anchors
        let fill_brush = context.solid_brush(WHITE);
        let fill_brush_active = context.solid_brush(PINK);

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
            RECT_BORDER_WIDTH,
        );

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
            RECT_BORDER_WIDTH,
        );

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
            RECT_BORDER_WIDTH,
        );

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
            RECT_BORDER_WIDTH,
        );

        // Stroked lines
        let mut line_stroke = StrokeStyle::new();
        line_stroke.set_dash(vec![5.0, 3.0], 0.0);
        let brush = context.solid_brush(PINK);

        context.stroke_styled(
            Line::new(
                (self.properties.line_left, 0.0),
                (self.properties.line_left, viewport_height),
            ),
            &brush,
            1.,
            &line_stroke,
        );

        context.stroke_styled(
            Line::new(
                (0.0, self.properties.line_top),
                (viewport_width, self.properties.line_top),
            ),
            &brush,
            1.,
            &line_stroke,
        );

        context.stroke_styled(
            Line::new(
                (self.properties.line_right, 0.0),
                (self.properties.line_right, viewport_height),
            ),
            &brush,
            1.,
            &line_stroke,
        );

        context.stroke_styled(
            Line::new(
                (0.0, self.properties.line_bottom),
                (viewport_width, self.properties.line_bottom),
            ),
            &brush,
            1.,
            &line_stroke,
        );
    }
}

fn get_properties(geometry: &RectGeometry) -> Properties {
    let border_width = geometry.width + RECT_BORDER_WIDTH;
    let border_height = geometry.height + RECT_BORDER_WIDTH;
    let border_left = geometry.x - RECT_BORDER_WIDTH / 2.;
    let border_top = geometry.y - RECT_BORDER_WIDTH / 2.;
    let border_right = border_left + border_width;
    let border_bottom = border_top + border_height;
    let line_left = geometry.x;
    let line_top = geometry.y;
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
        radius: RECT_ANCHOR_RADIUS + RECT_BORDER_WIDTH / 2.,
    }
}
