#[derive(Debug)]
pub struct Bicycle {make: String,
model: String,
size: i32,
color: String,
}


   
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
    pub fn with_make(self, make: &str) -> Self {
    Self {
    bicycle: Bicycle {
    make: make.into(),
    ..self.bicycle
    },
    }
    }
    pub fn with_model(self, model: &str) -> Self {
    Self {
    bicycle: Bicycle {
    model: model.into(),
    ..self.bicycle
    },
    }
    }
    pub fn with_size(self, size: i32) -> Self {
    Self {
    bicycle: Bicycle {
    size,
    ..self.bicycle
    },
    }
    }
    pub fn with_color(self, color: &str) -> Self {
    Self {
    bicycle: Bicycle {
    color: color.into(),
    ..self.bicycle} 
    }
}

pub fn build(self) -> Bicycle {self.bicycle
}
}
pub fn out(){
    let bicycle1 = Bicycle { 
        make: "Rivendell".into(),
        model: "A. Homer Hilsen".into(),
        size: 51,
        color: "red".into(),
        };
        println!("{:?}", bicycle1);
        let bicycle2 = Bicycle { 
        size: 58,
        ..bicycle1
        };
        println!("{:?}", bicycle2);
        // println!("{:?}", bicycle1); 

        let bicycle = BicycleBuilder::new()
.with_make("Trek")
.with_model("Madone")
.with_size(52)
.with_color("purple")
.build();
println!("{:?}", bicycle); 
    }