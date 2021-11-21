pub struct I64Pair {
    pub x: i64,
    pub y: i64,
}

pub struct I64Tri {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub trait Sort {
    fn sort(&mut self);
}

impl Sort for I64Pair {
    fn sort(&mut self)  {
        if self.y < self.x {
            let n = self.x;
            self.x = self.y;
            self.y = n;
        }
    }
}


impl Sort for I64Tri {
    fn sort(&mut self)  {
        if self.y < self.x {
            let n = self.x;
            self.x = self.y;
            self.y = n;
        }

        if self.z < self.y {
            let n = self.z;
            self.z = self.y;
            self.y = n;
        }

        if self.y < self.x {
            let n = self.x;
            self.x = self.y;
            self.y = n;
        }
    }
}
