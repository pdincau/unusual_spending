use mockall::*;

fn main() {
    println!("Hello, world!");
}

struct TriggerUnusualSpendingEmail {
    payments: Box<dyn Payments>,
}

impl TriggerUnusualSpendingEmail {
    pub fn trigger(&mut self, p0: &str) {
        self.payments.by_user_and_month();
    }
}

#[automock]
trait Payments {
    fn by_user_and_month(&mut self);
}

#[cfg(test)]
mod test {
    use crate::MockPayments;
    use crate::TriggerUnusualSpendingEmail;

    #[test]
    fn it_retrieves_payments_for_the_current_month() {
        let mut payments = MockPayments::new();

        payments
            .expect_by_user_and_month()
            .times(1)
            .returning(|| ());

        let mut application = TriggerUnusualSpendingEmail {
            payments: Box::new(payments),
        };

        application.trigger("aUserId");
    }
}
