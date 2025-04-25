#[derive(Debug)]
pub struct Bicycle {make: String,
model: String,
size: i32,
color: String,
}
// impl Bicycle { 
// fn make(&self) -> &String {
// &self.make
// }
// fn model(&self) -> &String {
// &self.model
// }
// fn size(&self) -> i32 {
// self.size
// }
// fn color(&self) -> &String {
// &self.color
// }
// }

   
#[derive(Debug)]
    pub struct BicycleBuilder {
        bicycle: Bicycle, 
        }
   

impl BicycleBuilder {
pub fn new() -> Self { 
Self {
bicycle: Bicycle {
make: String::new(),
model: String::new(),
size: 0,
color: String::new(),
},
}
}
pub fn with_make(&mut self, make: &str) { 
self.bicycle.make = make.into()
}
pub fn with_model(&mut self, model: &str) {
self.bicycle.model = model.into()
}
pub fn with_size(&mut self, size: i32) {
self.bicycle.size = size
}
pub fn with_color(&mut self, color: &str) {
self.bicycle.color = color.into()} 

pub fn build(self) -> Bicycle {self.bicycle
}
}