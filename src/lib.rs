extern crate nalgebra_glm as glm;

use glm::I32Vec2;

use std::ops::{Add, Sub};

type Color = [f32; 4];

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub position: I32Vec2,
    pub size: I32Vec2
}

impl Rect {
    pub fn new(position: I32Vec2, size: I32Vec2) -> Self {
        Rect {
            position, 
            size
        }
    }

    pub fn max(&self) -> I32Vec2 {
        self.position + self.size
    }

    pub fn contains(&self, point: I32Vec2) -> bool {
        let max = self.max();
        self.position.x <= point.x && point.x <= max.x
        && self.position.y <= point.y && point.y <= max.y
    }    

    pub fn shrink(&self, pxls: I32Vec2) -> Rect {
        Rect {
            position: self.position + pxls,
            size: self.size - pxls
        }
    }

    pub fn width(&self) -> i32 {
        self.size.x
    }

    pub fn height(&self) -> i32 {
        self.size.y
    }
}

type Id = &'static str;

pub enum DrawItem {
    Rect(Rect, Color)
}

pub struct Context {
    pub mouse_pos: I32Vec2,
    pub mouse_down: bool,
    pub hot: Option<Id>,
    pub active: Option<Id>,
    pub draw_list: Vec<DrawItem>
}

impl Context {
    pub fn new() -> Self {
        Context {
            mouse_pos: I32Vec2::zeros(),
            mouse_down: false,
            hot: None,
            active: None,
            draw_list: Vec::with_capacity(1000)
        }
    }
    pub fn mouse_pos(&self) -> I32Vec2 {
        self.mouse_pos
    }
    pub fn mouse_down(&self) -> bool {
        self.mouse_down
    }
    pub fn hot(&self) -> Option<Id> {
        self.hot
    }
    pub fn is_hot(&self, id: Id) -> bool {
        self.hot == Some(id)
    }
    pub fn is_active(&self, id: Id) -> bool {
        self.active == Some(id)
    }
    pub fn draw(&mut self, draw_item: DrawItem) {
        self.draw_list.push(draw_item)
    }
    pub fn set_hot(&mut self, id: Id) {
        self.hot = Some(id)
    }
    pub fn set_active(&mut self, id: Id) {
        self.active = Some(id)
    }
    pub fn is_hot_and_active(&self, id: Id) -> bool {
        self.hot == Some(id) 
        && self.active == Some(id)
    }
    pub fn draw_list_iter(&mut self) -> std::slice::Iter<DrawItem> {
        self.draw_list.iter()
    }

    pub fn update(&mut self, mouse_pos: I32Vec2, mouse_down: bool) {
        self.hot = None;
        if self.mouse_down() {
            self.active = Some("innactive");
        } else {
            self.active = None;
        }
        self.mouse_pos = mouse_pos;
        self.mouse_down = mouse_down;
        self.draw_list.clear();        
    }


}

pub fn button(ctx: &mut Context, id: Id, rect: Rect) -> bool {
    if rect.contains(ctx.mouse_pos()) {
        ctx.set_hot(id);
        if ctx.active.is_none() && ctx.mouse_down() {
            ctx.set_active(id)
        }
    }        
    
    ctx.draw(DrawItem::Rect(rect, [0., 0., 0., 0.]));
    let color = 
        if ctx.is_active(id) {
            [1., 1., 1., 1.]
        } else if ctx.is_hot(id) {
            [0.8, 0.8, 0.8, 1.]
        } else {
            [0.5, 0.5, 0.5, 1.]
        };        
    ctx.draw(DrawItem::Rect(rect.shrink(I32Vec2::new(2, 2)), color));    
    ctx.is_active(id) && !ctx.mouse_down() 
}

// pub fn vertical_slider(ctx: &mut Ctx, id: Id, rect: Rect, current_value: f32) -> f32 {
//     if rect.contains(ctx.mouse_pos()) {
//         ctx.set_focus(id)
//     }
//     ctx.draw_rect(rect, [0.4, 0.4, 0.4, 0.4]);        
//     let relative_mouse_y = ctx.mouse_pos().y - rect.position.y;
//     let max_y = rect.height();
//     let mouse_y_ratio = relative_mouse_y as f32 / max_y as f32;
//     let next_value = 
//         if ctx.focus() == Focus::Active(id) && ctx.mouse_down() {
//             mouse_y_ratio
//         } else {
//             current_value
//         };    
//     let slider_height = (max_y as f32 * next_value) as i32;
//     let slider_rect = 
//         Rect {         
//             position: rect.position + I32Vec2::new(0, slider_height),
//             size: I32Vec2::new(rect.width(), rect.width())
//         };
//     ctx.draw_rect(slider_rect, [1., 1., 1., 1.]);
//     next_value
// }




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

