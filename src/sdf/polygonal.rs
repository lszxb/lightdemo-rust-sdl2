use ::EPSILON;
use super::{Node, SDF};

pub struct Line<'a>(pub &'a Node, pub &'a Node);

impl<'a> SDF for Line<'a> {
    fn sdf(&self, p: &Node) -> f32 {
        let a = p.sdf(self.0);
        let b = p.sdf(self.1);
        let c = self.0.sdf(self.1);
        let tmp = a * a + c * c - b * b;
        if tmp < 0.0 {
            return a;
        }
        if b * b + c * c - a * a < 0.0 {
            return b;
        }

        (a * a - (tmp / 2.0 / c).powi(2)).sqrt()
    }
}

pub struct Capsule<'a>(pub Line<'a>, pub f32);

impl<'a> SDF for Capsule<'a> {
    fn sdf(&self, p: &Node) -> f32 {
        self.0.sdf(p) - self.1
    }
}

pub struct Polygonal<'a>(pub &'a [Node]);

impl<'a> SDF for Polygonal<'a> {
    fn sdf(&self, p: &Node) -> f32 {
        let mut d: f32 = 1.0 / EPSILON;
        for i in 0..self.0.len() - 1 {
            d = d.min(Line(&self.0[i], &self.0[i+1]).sdf(p));
        }
        d = d.min(Line(&self.0[self.0.len()-1], &self.0[0]).sdf(p));

        d //TODO: change unsigned to signed
    }
}

pub struct PolygonalCapsule<'a>(pub &'a [Node], pub f32);

impl<'a> SDF for PolygonalCapsule<'a> {
    fn sdf(&self, p: &Node) -> f32 {
        let mut d: f32 = 1.0 / EPSILON;
        for i in 0..self.0.len() - 1 {
            d = d.min(Capsule(Line(&self.0[i], &self.0[i+1]), self.1).sdf(p));
        }
        d = d.min(Capsule(Line(&self.0[self.0.len()-1], &self.0[0]), self.1).sdf(p));

        d //TODO: change unsigned to signed
    }
}
