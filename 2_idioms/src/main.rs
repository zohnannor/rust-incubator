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
    coins: BTreeMap<Coin, usize>,
    state: S,
}

impl Default for VendingMachine<Maintenance> {
    fn default() -> Self {
        Self {
            products: BTreeMap::default(),
            coins: BTreeMap::default(),
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
        let money = Money::from_amount(amount);
        Self::with_coins(money.0.into_iter())
    }

    fn with_coins(coins: impl IntoIterator<Item = Coin>) -> Self {
        let mut coins_map = BTreeMap::new();
        for coin in coins {
            coins_map
                .entry(coin)
                .and_modify(|amount| *amount += 1)
                .or_insert_with(|| 1);
        }

        Self {
            products: BTreeMap::new(),
            coins: coins_map,
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

    fn add_coins(&mut self, coins: impl IntoIterator<Item = Coin>) -> &mut Self {
        for coin in coins {
            self.coins
                .entry(coin)
                .and_modify(|amount| *amount += 1)
                .or_insert(1);
        }

        self
    }

    fn init(&mut self) -> VendingMachine<WaitingForUserChoice> {
        VendingMachine {
            products: mem::take(&mut self.products),
            coins: mem::take(&mut self.coins),
            state: WaitingForUserChoice,
        }
    }
}

impl VendingMachine<WaitingForUserChoice> {
    fn choose(self, name: ProductName) -> Result<VendingMachine<WaitingForCoin>, Error> {
        match self.products.get_key_value(&name) {
            Some((_, amount)) if *amount == 0 => Err(Error::NotEnoughProducts(Self {
                products: self.products,
                coins: self.coins,
                state: WaitingForUserChoice,
            })),
            Some((product, _)) => {
                let product = *product;
                Ok(VendingMachine {
                    products: self.products,
                    coins: self.coins,
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
        mut self,
    ) -> Result<(VendingMachine<WaitingForUserChoice>, Product, Option<Money>), Error> {
        let inserted = self.state.inserted.sum();
        let price = self.state.product.price;
        if inserted >= price {
            let rest = inserted - price; // can't overflow, inserted >= price

            let rest = self.calc_rest(rest)?;

            *self.products.get_mut(&self.state.product).unwrap() -= 1;

            Ok((
                VendingMachine {
                    products: self.products,
                    coins: self.coins,
                    state: WaitingForUserChoice,
                },
                self.state.product,
                rest,
            ))
        } else {
            Err(Error::NotEnoughMoney(
                VendingMachine {
                    products: self.products,
                    coins: self.coins,
                    state: WaitingForUserChoice,
                },
                self.state.inserted,
            ))
        }
    }

    fn calc_rest(&mut self, rest: usize) -> Result<Option<Money>, Error> {
        if rest == 0 {
            Ok(None)
        } else {
            let rest = Money::from_amount(rest);

            let machine_money = self
                .coins
                .iter()
                .flat_map(|(&c, &amount)| std::iter::repeat(c as u8 as usize).take(amount))
                .sum();

            if rest.sum() > machine_money {
                // trivial, rest is bigger than machine has
                return Err(Error::CantGiveRest(
                    VendingMachine {
                        products: mem::take(&mut self.products),
                        coins: mem::take(&mut self.coins),
                        state: WaitingForUserChoice,
                    },
                    mem::take(&mut self.state.inserted), // return them their money back
                ));
            }

            let mut machine_coins = self.coins.clone();

            for coin in &self.state.inserted.0 {
                machine_coins
                    .entry(*coin)
                    .and_modify(|amount| *amount += 1)
                    .or_insert(1);
            }

            for coin in &rest.0 {
                match machine_coins.get_mut(coin) {
                    Some(amount) => *amount -= 1,
                    None => {
                        return Err(Error::CantGiveRest(
                            VendingMachine {
                                products: mem::take(&mut self.products),
                                coins: mem::take(&mut self.coins),
                                state: WaitingForUserChoice,
                            },
                            mem::take(&mut self.state.inserted), // return them their money back
                        ));
                    }
                };
            }

            self.coins = machine_coins;

            Ok(Some(rest))
        }
    }
}

#[derive(Debug, Default)]
struct Money(Vec<Coin>);

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.sum() == other.sum()
    }
}

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

    fn sum(&self) -> usize {
        self.0.iter().map(|c| *c as u8 as usize).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, num_enum::TryFromPrimitive)]
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
    let vm = VendingMachine::with_money_amount(17)
        .add_products(vec![
            Product::new(ProductName::ChocoBar, 23),
            Product::new(ProductName::Chips, 19),
            Product::new(ProductName::Crackers, 3),
            Product::new(ProductName::Soda, 15),
        ])
        .add_product(Product::new(ProductName::Water, 5))
        .add_coins([Coin::One; 10])
        .init();

    let mut vm = vm.choose(ProductName::Chips).unwrap();
    vm.insert_coin(Coin::Ten);

    let (vm, _returned_money) = match vm.try_get_product() {
        Err(Error::NotEnoughMoney(vm, money)) => (vm, money),
        _ => unreachable!(),
    };

    let mut vm = vm.choose(ProductName::Chips).unwrap();
    vm.insert_coin(Coin::Ten);
    vm.insert_coin(Coin::Ten);
    let (_vm, product, rest) = vm.try_get_product().unwrap();
    assert_eq!(
        product,
        Product {
            name: ProductName::Chips,
            price: 19
        }
    );
    assert_eq!(rest, Some(Money::from_amount(1)));
}
