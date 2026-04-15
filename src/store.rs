use micro_moka::unsync::Cache;
use nu_protocol::{Record, Span, Value};

pub struct Store {
    cache: Cache<String, Value>,
    max_capacity: u64,
}

impl Store {
    pub fn new(max_capacity: u64) -> Self {
        Self {
            cache: Cache::new(max_capacity),
            max_capacity,
        }
    }

    pub fn put(&mut self, key: String, value: Value) {
        self.cache.insert(key, value);
    }

    pub fn put_if_absent(&mut self, key: String, value: Value) -> (bool, Value) {
        if let Some(existing) = self.cache.get(&key).cloned() {
            (false, existing)
        } else {
            self.cache.insert(key, value.clone());
            (true, value)
        }
    }

    pub fn get_or_put(&mut self, key: String, value: Value) -> Value {
        if let Some(existing) = self.cache.get(&key).cloned() {
            existing
        } else {
            self.cache.insert(key, value.clone());
            value
        }
    }

    pub fn get(&mut self, key: &str) -> Option<Value> {
        self.cache.get(key).cloned()
    }

    pub fn take(&mut self, key: &str) -> Option<Value> {
        self.cache.remove(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.cache.remove(key).is_some()
    }

    pub fn has(&mut self, key: &str) -> bool {
        self.cache.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.cache.invalidate_all();
    }

    pub fn stats(&mut self, span: Span) -> Value {
        let mut record = Record::new();
        record.push("entry_count", Value::int(self.cache.entry_count() as i64, span));
        record.push(
            "max_capacity",
            Value::int(self.max_capacity as i64, span),
        );

        Value::record(record, span)
    }

    pub fn incr(&mut self, key: String, delta: i64, span: Span) -> Result<i64, &'static str> {
        let next = match self.cache.get(&key) {
            Some(Value::Int { val, .. }) => val.saturating_add(delta),
            Some(_) => return Err("Value is not an integer"),
            None => delta,
        };

        self.cache.insert(key, Value::int(next, span));
        Ok(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get_take_roundtrip() {
        let span = Span::test_data();
        let mut store = Store::new(16);
        let value = Value::record(Record::from_iter([("name", Value::string("alice", span))]), span);

        store.put("k".into(), value.clone());

        assert_eq!(store.get("k"), Some(value.clone()));
        assert_eq!(store.take("k"), Some(value));
        assert_eq!(store.get("k"), None);
    }

    #[test]
    fn put_if_absent_reports_existing_value() {
        let span = Span::test_data();
        let mut store = Store::new(16);
        let first = Value::string("first", span);
        let second = Value::string("second", span);

        assert_eq!(store.put_if_absent("k".into(), first.clone()), (true, first.clone()));
        assert_eq!(store.put_if_absent("k".into(), second), (false, first));
    }

    #[test]
    fn get_or_put_returns_existing_value() {
        let span = Span::test_data();
        let mut store = Store::new(16);
        let first = Value::string("first", span);
        let second = Value::string("second", span);

        assert_eq!(store.get_or_put("k".into(), first.clone()), first.clone());
        assert_eq!(store.get_or_put("k".into(), second), first);
    }

    #[test]
    fn incr_initializes_and_increments_integer_values() {
        let span = Span::test_data();
        let mut store = Store::new(16);

        assert_eq!(store.incr("k".into(), 2, span), Ok(2));
        assert_eq!(store.incr("k".into(), 3, span), Ok(5));
    }

    #[test]
    fn incr_rejects_non_integer_values() {
        let span = Span::test_data();
        let mut store = Store::new(16);

        store.put("k".into(), Value::string("nope", span));

        assert_eq!(store.incr("k".into(), 1, span), Err("Value is not an integer"));
    }
}
