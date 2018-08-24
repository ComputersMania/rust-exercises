// struct Room {
//     name: String,
//     available: bool,
//     price: Option<f64>,
// }
//
// impl Room {
//     // Associated functions
//     fn new (name: String, price: Option<f64>) -> Room {
//         Room {
//             name,
//             available: true,
//             price,
//         }
//     }
//
//     // Methods
//     fn book (&mut self) {
//         if self.available {
//             match self.price {
//                 None => {
//                     println!("Aggratis la camera {}", self.name);
//                     self.available = false;
//                 },
//                 Some(price) => println!("Paga {}.", price),
//             }
//         }
//     }
// }

enum Room {
    Street(String),
    Estate{
        name: String,
        price: f64
    },
}

impl Room {
    // Associated functions
    fn new(name: String, price: Option<f64>) -> Room {
        match price {
            None => Room::Street(name),
            Some(price) => Room::Estate{name, price},
        }
    }

    // Methods
    fn book(&self) {
        match self {
            Room::Street => println!("Barbone!!"),
            Room::Estate => println!("Pagah {} euro!", self.price)
        }
    }
}


fn main() {
    let mut merda = Room::new(String::from("Schifosa"), None);
    let mut reale = Room::new(String::from("Estate"), Some(9000000.90));

    merda.book();
    reale.book();
}
