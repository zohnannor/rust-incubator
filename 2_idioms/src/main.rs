use std::mem;
use std::{borrow::Borrow, collections::BTreeMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum ProductName {
    ChocoBar,
    Chips,
    Crackers,
    Soda,
    Water,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Product {
    name: ProductName,
    price: usize,
}

impl Borrow<ProductName> for Product {
    fn borrow(&self) -> &ProductName {
        &self.name
    }
}

impl Product {
    const fn new(name: ProductName, price: usize) -> Self {
        Self { name, price }
    }
}

trait State {}

#[derive(Debug)]
struct Maintenance;
#[derive(Debug)]
struct WaitingForUserChoice;
#[derive(Debug)]
struct WaitingForCoin {
    product: Product,
    inserted: Money,
}

impl State for Maintenance {}
impl State for WaitingForUserChoice {}
impl State for WaitingForCoin {}

#[derive(Debug)]
struct VendingMachine<S> {
    products: BTreeMap<Product, usize>,
    money: Money,
    state: S,
}

impl Default for VendingMachine<Maintenance> {
    fn default() -> Self {
        Self {
            products: BTreeMap::default(),
            money: Money::default(),
            state: Maintenance,
        }
    }
}

#[derive(Debug)]
enum Error {
    NotEnoughProducts(VendingMachine<WaitingForUserChoice>),
    NotEnoughMoney(VendingMachine<WaitingForUserChoice>, Money),
    CantGiveRest(VendingMachine<WaitingForUserChoice>, Money),
}

impl VendingMachine<Maintenance> {
    fn with_money_amount(amount: usize) -> Self {
        Self {
            products: BTreeMap::new(),
            money: Money::from_amount(amount),
            state: Maintenance,
        }
    }

    fn with_coins(coins: impl IntoIterator<Item = Coin>) -> Self {
        Self {
            products: BTreeMap::new(),
            money: Money(coins.into_iter().collect()),
            state: Maintenance,
        }
    }

    fn add_product(&mut self, product: Product) -> &mut Self {
        self.products
            .entry(product)
            .and_modify(|amount| *amount += 1)
            .or_insert(1);
        self
    }

    fn add_products(&mut self, products: impl IntoIterator<Item = Product>) -> &mut Self {
        for product in products {
            self.add_product(product);
        }
        self
    }

    fn init(&mut self) -> VendingMachine<WaitingForUserChoice> {
        VendingMachine {
            products: mem::take(&mut self.products),
            money: mem::take(&mut self.money),
            state: WaitingForUserChoice,
        }
    }
}

impl VendingMachine<WaitingForUserChoice> {
    fn choose(self, name: ProductName) -> Result<VendingMachine<WaitingForCoin>, Error> {
        dbg!(&self);
        match self.products.get_key_value(&name) {
            Some((_, amount)) if *amount == 0 => Err(Error::NotEnoughProducts(Self {
                products: self.products,
                money: self.money,
                state: WaitingForUserChoice,
            })),
            Some((product, _)) => {
                let product = *product;
                Ok(VendingMachine {
                    products: self.products,
                    money: self.money,
                    state: WaitingForCoin {
                        product,
                        inserted: Money::default(),
                    },
                })
            }

            // TODO
            None => unreachable!(),
        }
    }
}

impl VendingMachine<WaitingForCoin> {
    fn insert_coin(&mut self, coin: Coin) {
        self.state.inserted.0.push(coin);
    }

    fn try_get_product(
        self,
    ) -> Result<(VendingMachine<WaitingForUserChoice>, Product, Option<Money>), Error> {
        let inserted = self
            .state
            .inserted
            .0
            .iter()
            .map(|c| *c as u8 as usize)
            .sum::<usize>();

        let rest = self.state.product.price.checked_sub(inserted);

        if rest.is_some() {}
        todo!()
    }
}

#[derive(Debug, Default)]
struct Money(Vec<Coin>);

impl Money {
    fn from_amount(mut amount: usize) -> Self {
        let mut coins = vec![];
        while amount != 0 {
            if let Some(new_amount) = amount.checked_sub(50) {
                coins.push(Coin::Fifty);
                amount = new_amount;
            } else if let Some(new_amount) = amount.checked_sub(20) {
                coins.push(Coin::Twenty);
                amount = new_amount;
            } else if let Some(new_amount) = amount.checked_sub(10) {
                coins.push(Coin::Ten);
                amount = new_amount;
            } else if let Some(new_amount) = amount.checked_sub(5) {
                coins.push(Coin::Five);
                amount = new_amount;
            } else if let Some(new_amount) = amount.checked_sub(2) {
                coins.push(Coin::Two);
                amount = new_amount;
            } else if let Some(new_amount) = amount.checked_sub(1) {
                coins.push(Coin::One);
                amount = new_amount;
            }
        }

        Self(coins)
    }
}

#[derive(Debug, Clone, Copy, num_enum::TryFromPrimitive)]
#[repr(u8)]
enum Coin {
    One = 1,
    Two = 2,
    Five = 5,
    Ten = 10,
    Twenty = 20,
    Fifty = 50,
}

fn main() {
    let vm = VendingMachine::with_money_amount(14)
        .add_products(vec![
            Product::new(ProductName::ChocoBar, 23),
            Product::new(ProductName::Chips, 19),
            Product::new(ProductName::Crackers, 3),
            Product::new(ProductName::Soda, 15),
        ])
        .add_product(Product::new(ProductName::Water, 5))
        .init();

    let mut vm = vm.choose(ProductName::Chips).unwrap();
    vm.insert_coin(Coin::Ten);

    let (vm, _money) = match vm.try_get_product() {
        Err(Error::NotEnoughMoney(vm, money)) => (vm, money),
        _ => unreachable!(),
    };

    let mut vm = vm.choose(ProductName::Chips).unwrap();
    vm.insert_coin(Coin::Ten);
    vm.insert_coin(Coin::Ten);
    let (vm, product, rest) = vm.try_get_product().expect("inserted required amount");
}
