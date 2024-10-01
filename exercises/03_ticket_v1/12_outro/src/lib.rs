// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
  product_name: String,
  quantity: i16,
  unit_price: i16
}

impl Order {
  pub fn new(name: String, qty: i16, price: i16) -> Order {
    name_ok(&name);
    qty_ok(qty);
    price_ok(price);
    Order {
      product_name: name,
      quantity: qty,
      unit_price: price
    }
  }

  //   Order must include a method named `total` that returns the total price of the order.
  pub fn total(&self) -> i16 {
    self.quantity * self.unit_price
  }

  //   Order must provide setters and getters for each field.
  pub fn product_name(&self) -> &String {
    &(self.product_name)
  }
  pub fn set_product_name(&mut self, new_name: String) {
    if name_ok(&new_name) {
      self.product_name = new_name
    }
  }
  pub fn quantity(&self) -> &i16 {
    &(self.quantity)
  }
  pub fn set_quantity(&mut self, new_qty: i16) {
    if qty_ok(new_qty) {
      self.quantity = new_qty
    }
  }
  pub fn unit_price(&self) -> &i16 {
    &(self.unit_price)
  }
  pub fn set_unit_price(&mut self, new_price: i16) {
    if price_ok(new_price) {
      self.unit_price = new_price
    }
  }

}

fn name_ok(name: &String) -> bool {
//   The product name can't be empty and it can't be longer than 300 bytes.
  if name.is_empty() { panic!("Name is required")}
  if name.len() > 300 { panic!("Name must be < 300") }
  true
}
fn qty_ok(qty: i16) -> bool {
    //   The quantity must be strictly greater than zero.
    if qty <= 0 { panic!("Quantity must be > 0")}
    true
}

fn price_ok(price: i16) -> bool {
  //   The unit price is in cents and must be strictly greater than zero.
  if price <= 0 { panic!("Price must be > 0")}
  true
}
