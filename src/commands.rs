use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, PipelineData, Record, Signature, Span, SyntaxShape, Type,
    Value,
};

use crate::UMokaPlugin;

pub struct UMoka;
pub struct Put;
pub struct PutIfAbsent;
pub struct Get;
pub struct GetOrPut;
pub struct Take;
pub struct Delete;
pub struct Has;
pub struct Clear;
pub struct Incr;
pub struct Stats;

impl PluginCommand for UMoka {
    type Plugin = UMokaPlugin;

    fn name(&self) -> &str {
        "umoka"
    }

    fn description(&self) -> &str {
        "A bounded in-memory key to value store."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Experimental)
    }

    fn extra_description(&self) -> &str {
        "Run `help umoka put` or `help umoka get` for subcommands."
    }

    fn run(
        &self,
        _plugin: &UMokaPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        keep_alive(engine)?;
        Err(LabeledError::new("Subcommand required")
            .with_help("Use a subcommand such as `put`, `put-if-absent`, `get`, or `take`.")
            .with_label("subcommand missing here", call.head))
    }
}

impl SimplePluginCommand for Put {
    type Plugin = UMokaPlugin;

    fn name(&self) -> &str {
        "umoka put"
    }

    fn description(&self) -> &str {
        "Store a value under a key."
    }

    fn signature(&self) -> Signature {
        Signature::build(SimplePluginCommand::name(self))
            .required("key", SyntaxShape::String, "The cache key.")
            .optional("value", SyntaxShape::Any, "Value to store.")
            .input_output_types(vec![(Type::Any, Type::Any), (Type::Nothing, Type::Any)])
            .category(Category::Experimental)
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                example: "umoka put demo {filename: a.ovpn, config: 'abc'}",
                description: "Store a value by key.",
                result: Some(Value::test_record(Record::from_iter([
                    ("filename".into(), Value::test_string("a.ovpn")),
                    ("config".into(), Value::test_string("abc")),
                ]))),
            },
            Example {
                example: "{filename: a.ovpn, config: 'abc'} | umoka put demo",
                description: "Store a piped value.",
                result: Some(Value::test_record(Record::from_iter([
                    ("filename".into(), Value::test_string("a.ovpn")),
                    ("config".into(), Value::test_string("abc")),
                ]))),
            },
        ]
    }

    fn run(
        &self,
        plugin: &UMokaPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        keep_alive(engine)?;
        let key: String = call.req(0)?;

        let value = resolve_value_input(call, input)?;

        let mut store = plugin.store().lock().map_err(|_| poison_error(call.head))?;
        store.put(key, value.clone());
        Ok(value)
    }
}

impl SimplePluginCommand for PutIfAbsent {
    type Plugin = UMokaPlugin;

    fn name(&self) -> &str {
        "umoka put-if-absent"
    }

    fn description(&self) -> &str {
        "Store a value only if the key is missing."
    }

    fn signature(&self) -> Signature {
        Signature::build(SimplePluginCommand::name(self))
            .required("key", SyntaxShape::String, "The cache key.")
            .optional("value", SyntaxShape::Any, "Value to store if missing.")
            .input_output_types(vec![
                (Type::Any, Type::Record(vec![].into())),
                (Type::Nothing, Type::Record(vec![].into())),
            ])
            .category(Category::Experimental)
    }

    fn run(
        &self,
        plugin: &UMokaPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        keep_alive(engine)?;
        let key: String = call.req(0)?;
        let value = resolve_value_input(call, input)?;

        let mut store = plugin.store().lock().map_err(|_| poison_error(call.head))?;
        let (inserted, stored_value) = store.put_if_absent(key, value);

        Ok(Value::record(
            Record::from_iter([
                ("inserted".into(), Value::bool(inserted, call.head)),
                ("value".into(), stored_value),
            ]),
            call.head,
        ))
    }
}

impl SimplePluginCommand for Get {
    type Plugin = UMokaPlugin;

    fn name(&self) -> &str {
        "umoka get"
    }

    fn description(&self) -> &str {
        "Get a value by key."
    }

    fn signature(&self) -> Signature {
        Signature::build(SimplePluginCommand::name(self))
            .required("key", SyntaxShape::String, "The cache key.")
            .input_output_type(Type::Nothing, Type::Any)
            .category(Category::Experimental)
    }

    fn run(
        &self,
        plugin: &UMokaPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        keep_alive(engine)?;
        let key: String = call.req(0)?;
        let mut store = plugin.store().lock().map_err(|_| poison_error(call.head))?;
        Ok(store.get(&key).unwrap_or_else(|| Value::nothing(call.head)))
    }
}

impl SimplePluginCommand for GetOrPut {
    type Plugin = UMokaPlugin;

    fn name(&self) -> &str {
        "umoka get-or-put"
    }

    fn description(&self) -> &str {
        "Return the existing value for a key, or store and return a fallback."
    }

    fn signature(&self) -> Signature {
        Signature::build(SimplePluginCommand::name(self))
            .required("key", SyntaxShape::String, "The cache key.")
            .optional("value", SyntaxShape::Any, "Value to store if missing.")
            .input_output_types(vec![(Type::Any, Type::Any), (Type::Nothing, Type::Any)])
            .category(Category::Experimental)
    }

    fn run(
        &self,
        plugin: &UMokaPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        keep_alive(engine)?;
        let key: String = call.req(0)?;
        let value = resolve_value_input(call, input)?;

        let mut store = plugin.store().lock().map_err(|_| poison_error(call.head))?;
        Ok(store.get_or_put(key, value))
    }
}

impl SimplePluginCommand for Take {
    type Plugin = UMokaPlugin;

    fn name(&self) -> &str {
        "umoka take"
    }

    fn description(&self) -> &str {
        "Get and delete a value by key."
    }

    fn signature(&self) -> Signature {
        Signature::build(SimplePluginCommand::name(self))
            .required("key", SyntaxShape::String, "The cache key.")
            .input_output_type(Type::Nothing, Type::Any)
            .category(Category::Experimental)
    }

    fn run(
        &self,
        plugin: &UMokaPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        keep_alive(engine)?;
        let key: String = call.req(0)?;
        let mut store = plugin.store().lock().map_err(|_| poison_error(call.head))?;
        Ok(store
            .take(&key)
            .unwrap_or_else(|| Value::nothing(call.head)))
    }
}

impl SimplePluginCommand for Delete {
    type Plugin = UMokaPlugin;

    fn name(&self) -> &str {
        "umoka delete"
    }

    fn description(&self) -> &str {
        "Delete a value by key."
    }

    fn signature(&self) -> Signature {
        Signature::build(SimplePluginCommand::name(self))
            .required("key", SyntaxShape::String, "The cache key.")
            .input_output_type(Type::Nothing, Type::Bool)
            .category(Category::Experimental)
    }

    fn run(
        &self,
        plugin: &UMokaPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        keep_alive(engine)?;
        let key: String = call.req(0)?;
        let mut store = plugin.store().lock().map_err(|_| poison_error(call.head))?;
        Ok(Value::bool(store.delete(&key), call.head))
    }
}

impl SimplePluginCommand for Has {
    type Plugin = UMokaPlugin;

    fn name(&self) -> &str {
        "umoka has"
    }

    fn description(&self) -> &str {
        "Check whether a key exists."
    }

    fn signature(&self) -> Signature {
        Signature::build(SimplePluginCommand::name(self))
            .required("key", SyntaxShape::String, "The cache key.")
            .input_output_type(Type::Nothing, Type::Bool)
            .category(Category::Experimental)
    }

    fn run(
        &self,
        plugin: &UMokaPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        keep_alive(engine)?;
        let key: String = call.req(0)?;
        let mut store = plugin.store().lock().map_err(|_| poison_error(call.head))?;
        Ok(Value::bool(store.has(&key), call.head))
    }
}

impl SimplePluginCommand for Clear {
    type Plugin = UMokaPlugin;

    fn name(&self) -> &str {
        "umoka clear"
    }

    fn description(&self) -> &str {
        "Delete all entries."
    }

    fn signature(&self) -> Signature {
        Signature::build(SimplePluginCommand::name(self))
            .input_output_type(Type::Nothing, Type::Bool)
            .category(Category::Experimental)
    }

    fn run(
        &self,
        plugin: &UMokaPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        keep_alive(engine)?;
        let mut store = plugin.store().lock().map_err(|_| poison_error(call.head))?;
        store.clear();
        Ok(Value::bool(true, call.head))
    }
}

impl SimplePluginCommand for Incr {
    type Plugin = UMokaPlugin;

    fn name(&self) -> &str {
        "umoka incr"
    }

    fn description(&self) -> &str {
        "Atomically increment an integer value by key."
    }

    fn signature(&self) -> Signature {
        Signature::build(SimplePluginCommand::name(self))
            .required("key", SyntaxShape::String, "The cache key.")
            .optional("delta", SyntaxShape::Int, "Amount to add. Defaults to 1.")
            .input_output_type(Type::Nothing, Type::Int)
            .category(Category::Experimental)
    }

    fn run(
        &self,
        plugin: &UMokaPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        keep_alive(engine)?;
        let key: String = call.req(0)?;
        let delta = call.opt::<i64>(1)?.unwrap_or(1);

        let mut store = plugin.store().lock().map_err(|_| poison_error(call.head))?;
        let next = store.incr(key, delta, call.head).map_err(|message| {
            LabeledError::new(message)
                .with_help("Use `umoka incr` only on integer values or on missing keys.")
                .with_label("integer value required here", call.head)
        })?;

        Ok(Value::int(next, call.head))
    }
}

impl SimplePluginCommand for Stats {
    type Plugin = UMokaPlugin;

    fn name(&self) -> &str {
        "umoka stats"
    }

    fn description(&self) -> &str {
        "Show store statistics."
    }

    fn signature(&self) -> Signature {
        Signature::build(SimplePluginCommand::name(self))
            .input_output_type(Type::Nothing, Type::Record(vec![].into()))
            .category(Category::Experimental)
    }

    fn run(
        &self,
        plugin: &UMokaPlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        keep_alive(engine)?;
        let mut store = plugin.store().lock().map_err(|_| poison_error(call.head))?;
        Ok(store.stats(call.head))
    }
}

fn is_nothing(value: &Value) -> bool {
    matches!(value, Value::Nothing { .. })
}

fn resolve_value_input(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    match (is_nothing(input), call.opt::<Value>(1)?) {
        (false, None) => Ok(input.clone()),
        (true, Some(value)) => Ok(value),
        (true, None) => Err(LabeledError::new("Missing value")
            .with_help("Pass a value as pipeline input or as the second positional argument.")
            .with_label("value required here", call.head)),
        (false, Some(_)) => Err(LabeledError::new("Ambiguous value input")
            .with_help("Use either pipeline input or a positional value, not both.")
            .with_label("conflicting inputs", call.head)),
    }
}

fn poison_error(span: Span) -> LabeledError {
    LabeledError::new("UMoka store lock poisoned").with_label("store unavailable", span)
}

fn keep_alive(engine: &EngineInterface) -> Result<(), LabeledError> {
    engine.set_gc_disabled(true).map_err(LabeledError::from)
}
