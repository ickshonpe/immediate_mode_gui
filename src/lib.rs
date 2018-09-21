use std::ops::{Add, Sub};

type Color = [f32; 4];

#[derive(Clone, Copy, Debug)]
pub struct Coords(pub i32, pub i32);

impl Add for Coords {
    type Output = Coords;
    fn add(self, other: Coords) -> Coords {
        Coords(self.0 + other.0, self.1 + other.1)        
    }
}

impl Sub for Coords {
    type Output = Coords;
    fn sub(self, other: Coords) -> Coords {
        Coords(self.0 - other.0, self.1 - other.1)        
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub position: Coords,
    pub size: Coords
}

impl Rect {
    pub fn new(position: Coords, size: Coords) -> Self {
        Rect {
            position, 
            size
        }
    }

    pub fn max(&self) -> Coords {
        self.position + self.size
    }

    pub fn contains(&self, point: Coords) -> bool {
        let max = self.max();
        self.position.0 <= point.0 && point.0 <= max.0
        && self.position.1 <= point.1 && point.1 <= max.1
    }    

    pub fn shrink(&self, pxls: Coords) -> Rect {
        Rect {
            position: self.position + pxls,
            size: self.position - pxls
        }
    }

    pub fn width(&self) -> i32 {
        self.size.0
    }

    pub fn height(&self) -> i32 {
        self.size.1
    }
}

type Id = &'static str;

#[derive(PartialEq)]
pub enum Focus {
    None,
    Hot(Id),
    Active(Id)
}

pub trait Ctx {
    fn mouse_pos(&self) -> Coords;
    fn mouse_down(&self) -> bool;
    fn draw_rect(&mut self, rect: Rect, color: Color);
    fn focus(&self) -> Focus;
    fn set_focus_state(&mut self, w: Focus);
    fn set_focus(&mut self, id: Id) {        
        let mouse_down = self.mouse_down();
        self.set_focus_state(
            if mouse_down {
                Focus::Active(id)
            } else {
                Focus::Hot(id)
            }
        )
    }
}

pub trait Renderer {
    fn draw_rect(&mut self, rect: Rect, color: Color);
}

pub fn button(ctx: &mut Ctx, id: Id, rect: Rect) -> bool {
    if rect.contains(ctx.mouse_pos()) {
        ctx.set_focus(id)
    }        
    ctx.draw_rect(rect, [0., 0., 0., 0.]);
    let color = 
        if ctx.focus() == Focus::Active(id) {
            [1., 1., 1., 1.]
        } else if ctx.focus() == Focus::Hot(id) {
            [0.8, 0.8, 0.8, 1.]
        } else {
            [0.5, 0.5, 0.5, 1.]
        };        
    ctx.draw_rect(rect.shrink(Coords(2, 2)), color);    
    ctx.focus() == Focus::Active(id) && !ctx.mouse_down() 
}

pub fn vertical_slider(ctx: &mut Ctx, id: Id, rect: Rect, current_value: f32) -> f32 {
    if rect.contains(ctx.mouse_pos()) {
        ctx.set_focus(id)
    }
    ctx.draw_rect(rect, [0.4, 0.4, 0.4, 0.4]);        
    let relative_mouse_y = ctx.mouse_pos().1 - rect.position.1;
    let max_y = rect.height();
    let mouse_y_ratio = relative_mouse_y as f32 / max_y as f32;
    let next_value = 
        if ctx.focus() == Focus::Active(id) && ctx.mouse_down() {
            mouse_y_ratio
        } else {
            current_value
        };    
    let slider_height = (max_y as f32 * next_value) as i32;
    let slider_rect = 
        Rect {         
            position: rect.position + Coords(0, slider_height),
            size: Coords(rect.width(), rect.width())
        };
    ctx.draw_rect(slider_rect, [1., 1., 1., 1.]);
    next_value
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

