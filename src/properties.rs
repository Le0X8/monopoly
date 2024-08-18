#[derive(Debug, Clone, Copy)]
pub enum Colors {
    Brown,
    LightBlue,
    Pink,
    Orange,
    Red,
    Yellow,
    Green,
    Blue,
}

use Colors::*;

impl Colors {
    pub fn next(&self) -> Colors {
        match self {
            Brown => LightBlue,
            LightBlue => Pink,
            Pink => Orange,
            Orange => Red,
            Red => Yellow,
            Yellow => Green,
            Green => Blue,
            Blue => Brown,
        }
    }
}

pub fn properties_per_color(color: &Colors) -> u8 {
    match color {
        Brown | Blue => 2,
        _ => 3,
    }
}

pub fn color_base_price(color: &Colors) -> u16 {
    match color {
        Brown => 60,
        LightBlue => 100,
        Pink => 140,
        Orange => 180,
        Red => 220,
        Yellow => 260,
        Green => 300,
        Blue => 350,
    }
}

pub fn color_base_rent(color: &Colors) -> u16 {
    match color {
        Brown => 2,
        LightBlue => 6,
        Pink => 10,
        Orange => 14,
        Red => 18,
        Yellow => 22,
        Green => 26,
        Blue => 35,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Property {
    pub color: Colors,
    pub no: u8,
    pub price: u16,
    pub houses: u8,
    pub mortgaged: bool,
    pub mortgaged_price: u16,
}

impl Property {
    pub fn first() -> Property {
        Property {
            color: Brown,
            no: 1,
            price: 60,
            houses: 0,
            mortgaged: false,
            mortgaged_price: 30,
        }
    }

    pub fn next(&self) -> Property {
        let ppc = properties_per_color(&self.color);
        let color;
        let no;
        if self.no == ppc {
            color = self.color.next();
            no = 1;
        } else {
            color = self.color;
            no = self.no + 1;
        }
        let price = get_price(&color, &no);
        Property {
            color,
            no,
            price,
            houses: 0,
            mortgaged: false,
            mortgaged_price: get_mortgaged_price(&price),
        }
    }

    pub fn get_house_price(&self) -> u16 {
        match self.color {
            Brown | LightBlue => 50,
            Pink | Orange => 100,
            Red | Yellow => 150,
            Green | Blue => 200,
        }
    }

    pub fn rent(&self) -> u16 {
        let ppc = properties_per_color(&self.color);
        let is_last = self.no == ppc;
        let base_rent = color_base_rent(&self.color);
        let rent_nohouses = if is_last {
            match self.color {
                Blue => 50,
                _ => base_rent + 2,
            }
        } else {
            base_rent
        };
        let house1_multiplier = match self.color {
            Green => {
                if is_last {
                    5.36
                } else {
                    5f64
                }
            }
            Blue => {
                if is_last {
                    4f64
                } else {
                    5f64
                }
            }
            _ => 5f64,
        };
        let house2_multiplier = match self.color {
            // todo
            _ => 15f64,
        };
        todo!()
    }
}

pub fn get_price(color: &Colors, no: &u8) -> u16 {
    let ppc = properties_per_color(color);
    let base_price = color_base_price(color);
    match color {
        Brown => base_price,
        _ => {
            base_price
                + if no == &ppc {
                    match color {
                        Blue => 50,
                        _ => 20,
                    }
                } else {
                    0
                }
        }
    }
}

pub fn get_mortgaged_price(price: &u16) -> u16 {
    price / 2
}

pub fn properties() -> [Property; 22] {
    let mut props = [Property::first(); 22];
    for i in 1..22 {
        props[i] = props[i - 1].next();
    }
    props
}
