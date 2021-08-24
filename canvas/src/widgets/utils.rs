use super::cursor::Cursor;
use nalgebra::geometry::Point2;
use ncollide2d::bounding_volume::{BoundingSphere, BoundingVolume};
use std::f64;

pub fn is_hovered(cursor: &Cursor, current_widget_bounding: &BoundingSphere<f64>) -> bool {
    let mut is_hovered = false;

    if let Some(cursor_position) = cursor.position {
        let current_cursor_bounding = BoundingSphere::new(cursor_position, 1.0);
        is_hovered =
            current_widget_bounding.intersects(&current_cursor_bounding) && !cursor.is_active;
    }

    is_hovered
}

pub fn is_selected_and_move(
    uuid: &str,
    active_widget_uuid: &Option<String>,
    cursor: &Cursor,
    initial_widget_bounding: &BoundingSphere<f64>,
) -> (bool, Option<Point2<f64>>) {
    let mut is_selected = false;
    let mut move_point = None;

    if let Some(state_uuid) = active_widget_uuid {
        // Set selected if it was already selected
        is_selected = uuid.eq(state_uuid);
    }

    if let Some(pos) = cursor.down_start_position {
        let initial_cursor_bounding = BoundingSphere::new(pos, 1.0);
        let is_initial_mouse_over = initial_widget_bounding.intersects(&initial_cursor_bounding);

        // Move (no anchors) | Difference of pointer movement
        if is_initial_mouse_over && !cursor.is_active {
            is_selected = true;

            if let Some(cursor_position) = cursor.position {
                if let Some(active_uuid) = active_widget_uuid {
                    if uuid == active_uuid {
                        move_point = Some(Point2::new(
                            cursor_position.x - pos.x,
                            cursor_position.y - pos.y,
                        ));
                    }
                }
            }
        }
    }

    (is_selected, move_point)
}
