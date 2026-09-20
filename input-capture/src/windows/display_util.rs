use windows::Win32::Foundation::RECT;

use crate::Position;

fn is_within_dp_region(point: (i32, i32), display: &RECT) -> bool {
    [
        Position::Left,
        Position::Right,
        Position::Top,
        Position::Bottom,
    ]
    .iter()
    .all(|&pos| is_within_dp_boundary(point, display, pos))
}

fn is_within_dp_boundary(point: (i32, i32), display: &RECT, pos: Position) -> bool {
    let (x, y) = point;
    match pos {
        Position::Left => display.left <= x,
        Position::Right => display.right > x,
        Position::Top => display.top <= y,
        Position::Bottom => display.bottom > y,
    }
}

/// returns whether the given position is within the display bounds with respect to the given
/// barrier position
///
/// # Arguments
///
/// * `x`:
/// * `y`:
/// * `displays`:
/// * `pos`:
///
/// returns: bool
///
fn in_bounds(point: (i32, i32), displays: &[RECT], pos: Position) -> bool {
    displays
        .iter()
        .any(|d| is_within_dp_boundary(point, d, pos))
}

fn in_display_region(point: (i32, i32), displays: &[RECT]) -> bool {
    displays.iter().any(|d| is_within_dp_region(point, d))
}

fn moved_across_boundary(
    prev_pos: (i32, i32),
    curr_pos: (i32, i32),
    displays: &[RECT],
    pos: Position,
) -> bool {
    /* was within bounds, but is not anymore */
    in_display_region(prev_pos, displays) && !in_bounds(curr_pos, displays, pos)
}

fn stalled_against_boundary(
    prev_pos: (i32, i32),
    curr_pos: (i32, i32),
    displays: &[RECT],
    pos: Position,
) -> bool {
    let stalled_on_axis = match pos {
        Position::Left | Position::Right => prev_pos.0 == curr_pos.0,
        Position::Top | Position::Bottom => prev_pos.1 == curr_pos.1,
    };
    if !stalled_on_axis || !in_display_region(curr_pos, displays) {
        return false;
    }

    // Windows clamps the cursor to the virtual desktop, so movement towards an
    // outer edge can produce repeated coordinates instead of an out-of-bounds
    // point. Check that the next pixel in that direction is outside every
    // display; this excludes edges shared by adjacent displays.
    let outside = match pos {
        Position::Left => (curr_pos.0 - 1, curr_pos.1),
        Position::Right => (curr_pos.0 + 1, curr_pos.1),
        Position::Top => (curr_pos.0, curr_pos.1 - 1),
        Position::Bottom => (curr_pos.0, curr_pos.1 + 1),
    };
    !in_display_region(outside, displays)
}

pub(crate) fn entered_barrier(
    prev_pos: (i32, i32),
    curr_pos: (i32, i32),
    displays: &[RECT],
) -> Option<Position> {
    [
        Position::Left,
        Position::Right,
        Position::Top,
        Position::Bottom,
    ]
    .into_iter()
    .find(|&pos| {
        moved_across_boundary(prev_pos, curr_pos, displays, pos)
            || stalled_against_boundary(prev_pos, curr_pos, displays, pos)
    })
}

///
/// clamp point to display bounds
///
/// # Arguments
///
/// * `prev_point`: coordinates the cursor had before entering
/// * `entry_point`: point to clamp
///
/// returns: (i32, i32), the corrected entry point
///
pub(crate) fn clamp_to_display_bounds(
    display_regions: &[RECT],
    prev_point: (i32, i32),
    point: (i32, i32),
) -> (i32, i32) {
    /*
     * Prefer the display where movement came from. Windows may report the
     * previous point just outside the desktop when an injected cursor first
     * reaches an edge, so fall back to the current (known-valid) point.
     */
    let display = display_regions
        .iter()
        .find(|&d| is_within_dp_region(prev_point, d))
        .or_else(|| {
            display_regions
                .iter()
                .find(|&d| is_within_dp_region(point, d))
        })
        .unwrap();

    /* clamp to bounds (inclusive) */
    let (x, y) = point;
    let (min_x, max_x) = (display.left, display.right - 1);
    let (min_y, max_y) = (display.top, display.bottom - 1);
    (x.clamp(min_x, max_x), y.clamp(min_y, max_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn display(left: i32, top: i32, right: i32, bottom: i32) -> RECT {
        RECT {
            left,
            top,
            right,
            bottom,
        }
    }

    #[test]
    fn detects_stalled_motion_at_each_outer_edge() {
        let displays = [display(0, 0, 100, 80)];

        assert_eq!(
            entered_barrier((0, 40), (0, 41), &displays),
            Some(Position::Left)
        );
        assert_eq!(
            entered_barrier((99, 40), (99, 41), &displays),
            Some(Position::Right)
        );
        assert_eq!(
            entered_barrier((40, 0), (41, 0), &displays),
            Some(Position::Top)
        );
        assert_eq!(
            entered_barrier((40, 79), (41, 79), &displays),
            Some(Position::Bottom)
        );
    }

    #[test]
    fn ignores_motion_away_from_an_edge() {
        let displays = [display(0, 0, 100, 80)];

        assert_eq!(entered_barrier((1, 40), (0, 40), &displays), None);
        assert_eq!(entered_barrier((0, 40), (1, 40), &displays), None);
        assert_eq!(entered_barrier((50, 40), (50, 41), &displays), None);
    }

    #[test]
    fn ignores_edges_shared_by_adjacent_displays() {
        let displays = [display(0, 0, 100, 80), display(100, 0, 200, 80)];

        assert!(!stalled_against_boundary(
            (99, 40),
            (99, 41),
            &displays,
            Position::Right
        ));
        assert!(!stalled_against_boundary(
            (100, 40),
            (100, 41),
            &displays,
            Position::Left
        ));
    }

    #[test]
    fn clamps_using_current_display_when_previous_point_is_outside() {
        let displays = [display(0, 0, 100, 80)];

        assert_eq!(
            clamp_to_display_bounds(&displays, (-1, 40), (0, 41)),
            (0, 41)
        );
        assert_eq!(
            clamp_to_display_bounds(&displays, (40, -1), (41, 0)),
            (41, 0)
        );
    }
}
