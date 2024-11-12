use crate::rect::Rect;

pub fn collide_rects(source: &Rect, target: &Rect) -> bool {
    let x_overlap =
        source.x < target.x + target.width as f32 && source.x + source.width as f32 > target.x;
    let y_overlap =
        source.y < target.y + target.height as f32 && source.y + source.height as f32 > target.y;

    x_overlap && y_overlap
}
