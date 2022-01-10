
pub enum Connectivity {
    NetworkUnit{x: [u8;4], y: [i32;6]},
    None,
}

pub struct NetworkUnit {
    pub ip: [u8; 4],
    pub mac: [i32; 6],
    link: Vec<Connectivity>,
}

impl NetworkUnit {
    pub fn init(i: [u8;4], m: [i32;6]) -> NetworkUnit {
        NetworkUnit { ip: i , mac: m , link: vec![Connectivity::None]} 
    }

    fn send() {

    }
    
    fn front_link_connection(&self , o: &NetworkUnit) -> Connectivity {
        Connectivity::NetworkUnit{x: o.ip, y: o.mac}
    }

    fn back_link_connection(&mut self, o: Connectivity) {
        match o {
            Connectivity::NetworkUnit{x,y} => self.link.push(Connectivity::NetworkUnit{x: x, y: y}),
            Connectivity::None => panic!("Error Connectivity"),
        }
        
    }
    
    pub fn link_connection(&mut self, local_o: &NetworkUnit) {
        self.back_link_connection(self.front_link_connection(local_o));
    }

    pub fn get_link(&self, i: usize) {
        match self.link[i] {
            Connectivity::NetworkUnit{x,y} => {println!("ip: {:?} | mac : {:?}", x , y)},
            Connectivity::None => {println!("Item with index {} isn't exist", i)},
        }
    }
}

fn main() {

    let mut testobject = NetworkUnit::init([192,168,0,1],[10,15,22,33,10,25]);
    let mut linkobject = NetworkUnit::init([192,168,0,2],[20,15,23,31,10,11]);
    testobject.link_connection(&linkobject);
    testobject.get_link(1);


    println!("End");
}
