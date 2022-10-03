pub type Cents = i64;

const CENTS_PER_AMOUNT: Cents = 10_000;

pub fn string_to_cents(s: &str) -> Result<Cents, &'static str> {
    let mut cents = 0;

    let negative = s.chars().next() == Some('-');

    let (amount, decimal) = match s.split_once('.') {
        Some((s1, s2)) => (s1, s2),
        None => (s, "0"),
    };

    // amount
    let amount = amount
        .parse::<Cents>()
        .map_err(|_| "Failed to parse amount")?;
    cents += amount * CENTS_PER_AMOUNT;

    // decimal
    if decimal.len() > 4 {
        return Err("Amount has too many decimal places");
    }
    let decimal = format!("{:0<4}", decimal);
    let mut decimal = decimal
        .parse::<Cents>()
        .map_err(|_| "Failed to parse decimal amount")?;
    if negative {
        decimal *= -1;
    }
    cents += decimal;

    Ok(cents)
}

pub fn cents_to_string(c: Cents) -> String {
    let sign = if c >= 0 { "" } else { "-" };
    let c = c.abs();

    let amount = c / CENTS_PER_AMOUNT;
    let decimal = c % CENTS_PER_AMOUNT;

    let decimal_fmt = format!("{:0>4}", decimal);
    let decimal_fmt = decimal_fmt.trim_end_matches('0');

    if decimal > 0 {
        format!("{}{}.{}", sign, amount, decimal_fmt)
    } else {
        format!("{}{}", sign, amount)
    }
}

#[cfg(test)]
mod tests {
    mod string_to_cents {
        use crate::cents::string_to_cents;

        #[test]
        fn invalid() {
            assert!(string_to_cents("abc").is_err());
            assert!(string_to_cents("1.1.1").is_err());
            assert!(string_to_cents("1.a").is_err());
            assert!(string_to_cents("0.12345").is_err());
        }

        #[test]
        fn whole_amounts() {
            assert_eq!(string_to_cents("-123"), Ok(-123_0000));
            assert_eq!(string_to_cents("0"), Ok(0));
            assert_eq!(string_to_cents("1"), Ok(1_0000));
            assert_eq!(string_to_cents("10"), Ok(10_0000));
            assert_eq!(string_to_cents("123"), Ok(123_0000));
        }

        #[test]
        fn with_decimals() {
            assert_eq!(string_to_cents("-1.01"), Ok(-1_0100));
            assert_eq!(string_to_cents("-0.1"), Ok(-0_1000));
            assert_eq!(string_to_cents("0.0123"), Ok(0_0123));
            assert_eq!(string_to_cents("0.1"), Ok(0_1000));
            assert_eq!(string_to_cents("0.1234"), Ok(0_1234));
            assert_eq!(string_to_cents("1.01"), Ok(1_0100));
        }
    }

    mod cents_to_string {
        use super::super::cents_to_string;

        #[test]
        fn whole_amounts() {
            assert_eq!(cents_to_string(-123_0000), "-123");
            assert_eq!(cents_to_string(0), "0");
            assert_eq!(cents_to_string(1_0000), "1");
            assert_eq!(cents_to_string(10_0000), "10");
            assert_eq!(cents_to_string(123_0000), "123");
        }

        #[test]
        fn with_decimals() {
            assert_eq!(cents_to_string(-1_0100), "-1.01");
            assert_eq!(cents_to_string(-0_1000), "-0.1");
            assert_eq!(cents_to_string(0_0123), "0.0123");
            assert_eq!(cents_to_string(0_1000), "0.1");
            assert_eq!(cents_to_string(0_1234), "0.1234");
            assert_eq!(cents_to_string(1_0100), "1.01");
        }
    }
}
