use mockall::*;

fn main() {
    println!("Hello, world!");
}

struct TriggerUnusualSpendingEmail {
    payments: Box<dyn Payments>,
}

impl TriggerUnusualSpendingEmail {
    pub fn new(payments: Box<dyn Payments>) -> Self {
        Self { payments }
    }

    pub fn trigger(&self, user_id: &str) {
        self.payments.by_user_and_period(user_id, 2023, 3);
    }
}

#[automock]
trait Payments {
    fn by_user_and_period(&self, user_id: &str, year: u32, month: u8) -> Vec<Payment>;
}

struct Payment {
    price: String,
    description: String,
    category: String,
}

impl Payment {
    pub fn new(price: &str, description: &str, category: &str) -> Self {
        Self {
            price: price.to_string(),
            description: description.to_string(),
            category: category.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_retrieves_payments_for_a_user_and_period() {
        let a_user_id = "aUserId";
        let a_payment = Payment::new("price", "description", "category");

        let mut payments = MockPayments::new();

        payments
            .expect_by_user_and_period()
            .times(1)
            .return_once(|user_id: &str, _, _| vec![a_payment]);

        let application = TriggerUnusualSpendingEmail::new(Box::new(payments));

        application.trigger(a_user_id);
    }
}

//TODO
// 1. return 1 payment
// 2. return result from payments
