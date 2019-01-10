use std::collections::HashMap;

use num_bigint::BigUint;

type BalanceHash = HashMap<String, BigUint>;

pub fn is_valid_transition(channel_deposit: &BigUint, prev: &BalanceHash, next: &BalanceHash) -> bool {
    let next_sum: BigUint = next.values().sum();

    if next_sum >= *channel_deposit {
        return false;
    }

    if prev.values().sum::<BigUint>() > next_sum {
        return false;
    }

    prev.iter().all(|(id, val)| next.contains_key(id) && next.get(id).unwrap() >= val)
}

pub fn get_health(our_balances: &BalanceHash, new_balances: &BalanceHash) -> bool {
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
        let our = our_balances.get(key).unwrap();
        let approved = new_balances.get(key).unwrap();

        // use an `if` rather than `min`, so that we can clone only one of the BigUint's
        if our > approved {
            approved.clone()
        } else {
            our.clone()
        }
    }).sum();

    sum_of_min * 1000u32 / sum_our_balance >= threshold
}

#[cfg(test)]
mod tests {
    mod is_valid_transition {
        use num_bigint::BigUint;

        use crate::is_valid_transition;
        use crate::BalanceHash;

        #[test]
        fn sum_of_next_values_should_not_exceed_the_channel_deposit() {
            let channel_deposit = BigUint::from(20u32);
            let mut next = BalanceHash::new();

            next.insert("a".to_owned(), BigUint::from(10u64));
            let prev = BalanceHash::new();

            assert_eq!(true, is_valid_transition(&channel_deposit, &prev, &next));

            // make the sum of next 30 which is > 20, it should fail the validation
            next.insert("b".to_owned(), BigUint::from(20u64));
            assert_eq!(false, is_valid_transition(&channel_deposit, &prev, &next));
        }

        #[test]
        fn sum_of_next_values_should_be_ge_than_sum_of_prev_values() {
            let channel_deposit = BigUint::from(100u32);

            let mut next = BalanceHash::new();
            next.insert("a".to_owned(), BigUint::from(30u64));
            next.insert("b".to_owned(), BigUint::from(10u64));

            let mut prev = BalanceHash::new();
            prev.insert("a".to_owned(), BigUint::from(30u64));
            prev.insert("b".to_owned(), BigUint::from(10u64));

            assert_eq!(true, is_valid_transition(&channel_deposit, &prev, &next));

            prev.insert("e".to_owned(), BigUint::from(20u64));
            // no entry for "e", so it should fail t he validation
            assert_eq!(false, is_valid_transition(&channel_deposit, &prev, &next));

            // make the sum of prev 50 which is > 30, it should fail the validation
            next.insert("e".to_owned(), BigUint::from(20u64));
            assert_eq!(true, is_valid_transition(&channel_deposit, &prev, &next));

            // make next "e" key smaller than the prev "e" key, it should fail the validation
            *next.get_mut(&"e".to_owned()).unwrap() = BigUint::from(10u64);
            // but also add another key to make the sum of next > prev!
            next.insert("f".to_owned(), BigUint::from(30u64));
            assert_eq!(false, is_valid_transition(&channel_deposit, &prev, &next));
        }
    }

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

            assert_eq!(true, get_health(&our, &approved));
        }

        #[test]
        fn approved_balance_greater_to_out_accounting_and_is_healthy() {
            let our_amount: BigUint = BigUint::from(50u64);
            let approved_amount = BigUint::from(60u64);

            let mut our = BalanceHash::new();
            our.insert("a".to_owned(), our_amount);

            let mut approved = BalanceHash::new();
            approved.insert("a".to_owned(), approved_amount);

            assert_eq!(true, get_health(&our, &approved));
        }

        #[test]
        fn approved_balance_has_less_than_our_accounting_but_is_healthy_because_of_margin() {
            let our_amount: BigUint = BigUint::from(80u64);
            let approved_amount = BigUint::from(79u64);

            let mut our = BalanceHash::new();
            our.insert("a".to_owned(), our_amount);

            let mut approved = BalanceHash::new();
            approved.insert("a".to_owned(), approved_amount);

            assert_eq!(true, get_health(&our, &approved));
        }

        #[test]
        fn approved_balance_has_less_than_our_accounting_and_is_unhealthy() {
            let our_amount: BigUint = BigUint::from(80u64);
            let approved_amount = BigUint::from(70u64);

            let mut our = BalanceHash::new();
            our.insert("a".to_owned(), our_amount);

            let mut approved = BalanceHash::new();
            approved.insert("a".to_owned(), approved_amount);

            assert_eq!(false, get_health(&our, &approved));
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
            assert_eq!(true, get_health(&our, &approved));
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
            assert_eq!(false, get_health(&our, &approved));
        }
    }
}
