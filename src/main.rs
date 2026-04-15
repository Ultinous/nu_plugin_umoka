use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_plugin_umoka::UMokaPlugin;

fn main() {
    serve_plugin(&mut UMokaPlugin::new(), MsgPackSerializer {})
}
