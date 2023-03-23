use mockall::*;

fn main() {
    println!("Hello, world!");
}

struct TriggerUnusualSpendingEmail {
    payments: Box<dyn Payments>,
}

impl TriggerUnusualSpendingEmail {
    pub fn trigger(&mut self, user_id: &str) {
        self.payments.by_user_and_period(user_id, 2023, 3);
    }
}

#[automock]
trait Payments {
    fn by_user_and_period(&mut self, user_id: &str, year: u32, month: u8);
}

#[cfg(test)]
mod test {
    use crate::MockPayments;
    use crate::TriggerUnusualSpendingEmail;

    #[test]
    fn it_retrieves_payments_for_the_current_month() {
        let a_user_id = "aUserId";

        let mut payments = MockPayments::new();

        payments
            .expect_by_user_and_period()
            .times(1)
            .returning(|_, _, _| ());

        let mut application = TriggerUnusualSpendingEmail {
            payments: Box::new(payments),
        };

        application.trigger(a_user_id);
    }
}
