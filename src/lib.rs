use std::collections::HashMap;

use num_bigint::BigUint;

type BalanceHash = HashMap<String, BigUint>;

pub fn get_health(our_balances: BalanceHash, new_balances: BalanceHash) -> bool {
    let threshold: BigUint = BigUint::from(950u32);
    let sum_our_balance: BigUint = our_balances.values().sum();

    let intersect_keys: Vec<String> = our_balances.iter().filter_map(|(id, _amount)| {
        if new_balances.contains_key(id) {
            return Some(id.clone());
        }

        None
    }).collect();

    let sum_of_min: BigUint = intersect_keys.iter().map(|key| {
        // return the minimum of the two values
        let our = our_balances.get(key).unwrap().clone();
        let approved = new_balances.get(key).unwrap().clone();

        BigUint::min(our, approved)
    }).sum();

    sum_of_min * 1000u32 / sum_our_balance >= threshold
}

#[cfg(test)]
mod tests {
    mod get_heath {
        use num_bigint::BigUint;

        use crate::get_health;
        use crate::BalanceHash;

        #[test]
        fn approved_balance_equals_to_out_accounting_and_is_healthy() {
            let our_amount = BigUint::from(50u64);
            let approved_amount = BigUint::from(50u64);

            let mut our = BalanceHash::new();
            our.insert("a".to_owned(), our_amount);

            let mut approved = BalanceHash::new();
            approved.insert("a".to_owned(), approved_amount);

            assert_eq!(true, get_health(our, approved));
        }

        #[test]
        fn approved_balance_greater_to_out_accounting_and_is_healthy() {
            let our_amount: BigUint = BigUint::from(50u64);
            let approved_amount = BigUint::from(60u64);

            let mut our = BalanceHash::new();
            our.insert("a".to_owned(), our_amount);

            let mut approved = BalanceHash::new();
            approved.insert("a".to_owned(), approved_amount);

            assert_eq!(true, get_health(our, approved));
        }

        #[test]
        fn approved_balance_has_less_than_our_accounting_but_is_healthy_because_of_margin() {
            let our_amount: BigUint = BigUint::from(80u64);
            let approved_amount = BigUint::from(79u64);

            let mut our = BalanceHash::new();
            our.insert("a".to_owned(), our_amount);

            let mut approved = BalanceHash::new();
            approved.insert("a".to_owned(), approved_amount);

            assert_eq!(true, get_health(our, approved));
        }

        #[test]
        fn approved_balance_has_less_than_our_accounting_and_is_unhealthy() {
            let our_amount: BigUint = BigUint::from(80u64);
            let approved_amount = BigUint::from(70u64);

            let mut our = BalanceHash::new();
            our.insert("a".to_owned(), our_amount);

            let mut approved = BalanceHash::new();
            approved.insert("a".to_owned(), approved_amount);

            assert_eq!(false, get_health(our, approved));
        }

        #[test]
        fn list_of_approved_balance_has_less_than_our_accounting_but_is_healthy_because_of_margin() {
            let mut our = BalanceHash::new();
            our.insert("a".to_owned(), BigUint::from(55u64));
            our.insert("b".to_owned(), BigUint::from(5u64));
            our.insert("c".to_owned(), BigUint::from(40u64));

            let mut approved = BalanceHash::new();
            approved.insert("a".to_owned(), BigUint::from(70u64));
            approved.insert("c".to_owned(), BigUint::from(40u64));

            // sum of mins 95 is 95% of 100 => not healthy
            assert_eq!(true, get_health(our, approved));
        }

        #[test]
        fn list_of_approved_balances_has_less_than_our_accounting_and_is_unhealthy() {
            let mut our = BalanceHash::new();
            our.insert("a".to_owned(), BigUint::from(10u64));
            our.insert("b".to_owned(), BigUint::from(20u64));
            our.insert("c".to_owned(), BigUint::from(30u64));

            let mut approved = BalanceHash::new();
            approved.insert("a".to_owned(), BigUint::from(20u64));
            approved.insert("c".to_owned(), BigUint::from(20u64));

            // sum of mins 30 is 50% of 60 => not healthy
            assert_eq!(false, get_health(our, approved));
        }
    }
}
