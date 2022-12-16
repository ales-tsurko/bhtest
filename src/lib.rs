use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum NetworkName {
    N1,
    N2,
    N3,
}

const TRACK: &[(&str, NetworkName)] = &[
    ("S1", NetworkName::N1),
    ("S2", NetworkName::N2),
    ("s3", NetworkName::N3),
];

fn test(tickers: Vec<Ticker>) -> HashMap<NetworkName, (u16, f32)> {
    let mut symbols_cache = HashMap::new();

    tickers
        .into_iter()
        .fold(HashMap::<NetworkName, (u16, f32)>::new(), |mut res, val| {
            let name = name_from_symbol(&val.symbol, &mut symbols_cache);
            let entry = res.entry(name).or_insert_with(Default::default);
            entry.0 += 1;
            entry.1 += val.price;
            res
        })
        .into_iter()
        .map(|(key, (count, price))| {
            let value = (count, price / count as f32);
            (key, value)
        })
        .collect()
}

fn name_from_symbol(symbol: &str, cache: &mut HashMap<&str, NetworkName>) -> NetworkName {
    // in a real-world application we'd rather use `FromStr` implementation for `NetworkName`. Or a
    // `HashMap` with pre-filled values. But per requirements we need to use the slice and show, how
    // to optimize it.

    // this is not very rusty, but the most optimized one, because the keys inside TRACK are
    // 'static. With methods like `.entry(...).insert_...` we couldn't just use keys from TRACK, as
    // we would need to clone the argument (`symbol`).
    if !cache.contains_key(symbol) {
        let (key, value) = TRACK
            .iter()
            .find(|(sym, _)| *sym == symbol)
            .expect("symbol is valid");
        cache.insert(key, *value);
    }

    cache[symbol]
}

struct Ticker {
    symbol: String,
    price: f32,
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::*;
    use assert_float_eq::{
        afe_abs, afe_absolute_error_msg, afe_is_absolute_eq, assert_float_absolute_eq,
    };

    #[test]
    fn test_correct() {
        let tickers = vec![
            Ticker {
                symbol: String::from("S1"),
                price: 0.1,
            },
            Ticker {
                symbol: String::from("S1"),
                price: 0.2,
            },
            Ticker {
                symbol: String::from("S1"),
                price: 0.3,
            },
            Ticker {
                symbol: String::from("S2"),
                price: 0.4,
            },
            Ticker {
                symbol: String::from("S2"),
                price: 0.5,
            },
            Ticker {
                symbol: String::from("S2"),
                price: 0.6,
            },
            Ticker {
                symbol: String::from("s3"),
                price: 0.7,
            },
            Ticker {
                symbol: String::from("s3"),
                price: 0.8,
            },
        ];
        let result = test(tickers);

        assert_eq!(3, result[&NetworkName::N1].0);
        assert_eq!(3, result[&NetworkName::N2].0);
        assert_eq!(2, result[&NetworkName::N3].0);

        assert_float_absolute_eq!(0.2, result[&NetworkName::N1].1, f32::EPSILON);
        assert_float_absolute_eq!(0.5, result[&NetworkName::N2].1, f32::EPSILON);
        assert_float_absolute_eq!(0.75, result[&NetworkName::N3].1, f32::EPSILON);
    }

    #[test]
    fn name_from_symbol_correct() {
        let mut cache = HashMap::new();

        assert_eq!(NetworkName::N1, name_from_symbol("S1", &mut cache));
        assert_eq!(NetworkName::N2, name_from_symbol("S2", &mut cache));
        assert_eq!(NetworkName::N3, name_from_symbol("s3", &mut cache));
    }

    #[test]
    fn name_from_symbol_cache() {
        let mut cache = HashMap::new();

        let now = Instant::now();
        name_from_symbol("S1", &mut cache);
        let before_cache = now.elapsed();

        let now = Instant::now();
        name_from_symbol("S1", &mut cache);
        let after_cache = now.elapsed();

        assert!(after_cache < before_cache);
    }

    #[test]
    #[should_panic]
    fn name_from_symbol_panics_on_invalid_symbol() {
        let mut cache = HashMap::new();
        name_from_symbol("foo", &mut cache);
    }
}
