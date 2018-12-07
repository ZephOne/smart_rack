use std::collections::HashMap;
struct Order {
    items_to_pick: HashMap<String, u8>,
    items_picked: HashMap<String, u8>,
    gen_code: String,
}

#[derive(Debug)]
struct Item {
    gen_code: String,
}

struct OrderList {
    order_list: HashMap<String, Order>,
}

struct Rack {
    current_order: Order,
    shelves: HashMap<String, Vec<Item>>,
    shelf_config: HashMap<u32, String>,
    led_config: HashMap<String, u32>,
}

impl Order {
    pub fn set_items_to_pick(&mut self, items_to_pick: HashMap<String, u8>) {
        self.items_to_pick = items_to_pick;
    }

    pub fn get_items_to_pick(&self) -> &HashMap<String, u8> {
        &self.items_to_pick
    }

    pub fn set_items_picked(&mut self, items_picked: HashMap<String, u8>) {
        self.items_picked = items_picked;
    }

    pub fn get_items_picked(&self) -> &HashMap<String, u8> {
        &self.items_picked
    }

    pub fn set_gen_code(&mut self, gen_code: String) {
        self.gen_code = gen_code;
    }

    pub fn get_gen_code(&self) -> &String {
        &self.gen_code
    }
}

impl Item {
    pub fn set_gen_code(&mut self, gen_code: String) {
        self.gen_code = gen_code;
    }

    pub fn get_gen_code(&self) -> &String {
        &self.gen_code
    }
}

impl OrderList {
    pub fn set_order_list(&mut self, order_list: HashMap<String, Order>) {
        self.order_list = order_list;
    }

    pub fn update(&mut self, order: Order) {
        self.order_list.insert(order.gen_code, order);
    }
}

impl Rack {
    pub fn set_current_order(&mut self, order: Order) {
        self.current_order = order;
    }

    pub fn state_changed(&mut self, item_number: u32) {
        if let Some(item_type) = self.shelf_config.get(&item_number) {
            if let Some(items) = self.shelves.get_mut(item_type) {
                let item_picked = items.remove(0);
                let item_picked_gen_code = item_picked.get_gen_code();
                match self
                    .current_order
                    .items_to_pick
                    .contains_key(item_picked_gen_code)
                {
                    true => match self.current_order.items_to_pick.get(item_picked_gen_code) {
                        Some(quantity) => {
                            if *quantity > 0 {
                                self.current_order
                                    .items_to_pick
                                    .insert(item_picked.gen_code.clone(), *quantity - 1);
                                println!("Quantity > 0");

                                if !self
                                    .current_order
                                    .items_picked
                                    .contains_key(item_picked_gen_code)
                                {
                                    self.current_order
                                        .items_picked
                                        .insert(item_picked.gen_code.clone(), 0);
                                }

                                self.current_order.items_picked.insert(
                                    item_picked.gen_code.clone(),
                                    self.current_order
                                        .items_picked
                                        .get(item_picked.get_gen_code())
                                        .unwrap()
                                        + 1,
                                );
                            }
                        }
                        _ => {
                            println!("Can't take one out");
                        }
                    },
                    false => println!("No item: {:?}", item_picked),
                }
            }
        }
    }
}
