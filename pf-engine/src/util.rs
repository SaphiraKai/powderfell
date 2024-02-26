use line_drawing::Bresenham;

use crate::{Particle, Playfield};

pub fn checked_move(p: &mut Particle, xoff: i32, yoff: i32, f: &mut Playfield) -> bool {
    let (h, w) = (f.height as i32, f.width as i32);
    let (x0, y0) = (p.x as i32, p.y as i32);
    let (x1, y1) = (x0 + xoff, y0 + yoff);

    let mut prev = (p.x, p.y);
    for (x, y) in Bresenham::new((x0, y0), (x1, y1)).skip(1).step_by(2) {
        let target = f.get(x as usize, y as usize).cloned();
        if x >= w || x < 0 || y >= h || y < 0 || target.is_some() {
            if target
                .as_ref()
                .is_some_and(|t| t.element.density() < p.element.density())
            {
                let t = target.clone().unwrap();

                t.move_to(prev.0, prev.1, f);
            } else {
                if prev == (p.x, p.y) {
                    return false;
                }

                p.move_to(prev.0, prev.1, f);
                return true;
            }
        }

        prev = (x as usize, y as usize);
    }

    p.move_to(prev.0, prev.1, f);

    true
}
