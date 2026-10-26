
struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn purchase_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850000,
    };

    let quantity = 3;

    let hp_cost = hp.purchase_cost(quantity);
    let ibm_cost = ibm.purchase_cost(quantity);
    let toshiba_cost = toshiba.purchase_cost(quantity);
    let dell_cost = dell.purchase_cost(quantity);

    println!("{} cost: N{}", hp.brand, hp_cost);
    println!("{} cost: N{}", ibm.brand, ibm_cost);
    println!("{} cost: N{}", toshiba.brand, toshiba_cost);
    println!("{} cost: N{}", dell.brand, dell_cost);

    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("-----------------------------");
    println!("Total cost: N{}", total_cost);
}
