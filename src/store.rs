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
}
