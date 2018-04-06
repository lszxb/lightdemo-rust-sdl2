pub mod polygonal;

use ::std::ops::{Add, Mul};

pub trait SDF {
    fn sdf(&self, p: &Node) -> f32;
}

#[derive(Clone, Copy)]
pub struct Node {
    pub x: f32,
    pub y: f32
}

impl Node {
    pub const fn new(x: f32, y: f32) -> Node {
        Node {
            x,
            y
        }
    }
}

impl SDF for Node {
    fn sdf(&self, p: &Node) -> f32 {
        let ux = p.x - self.x;
        let uy = p.y - self.y;
        (ux * ux + uy * uy).sqrt()
    }
}

impl<'a> Add<Node> for &'a Node {
    type Output = Node;
    fn add(self, p: Node) -> Node {
        Node {
            x: self.x + p.x,
            y: self.y + p.y
        }
    }
}

impl<'a> Add for &'a Node {
    type Output = Node;
    fn add(self, p: &'a Node) -> Node {
        Node {
            x: self.x + p.x,
            y: self.y + p.y
        }
    }
}

impl<'a> Mul<f32> for &'a Node {
    type Output = Node;
    fn mul(self, t: f32) -> Node {
        Node {
            x: self.x * t,
            y: self.y * t
        }
    }

}

pub struct Circle {
    pub c: Node,
    pub r: f32
}

impl SDF for Circle {
    fn sdf(&self, p: &Node) -> f32 {
        let ux = p.x - self.c.x;
        let uy = p.y - self.c.y;
        (ux * ux + uy * uy).sqrt() - self.r
    }
}

impl Circle {
    pub const fn new(cx: f32, cy: f32, r: f32) -> Circle {
        Circle {
            c: Node {
                x: cx,
                y: cy
            },
            r
        }
    }
}
