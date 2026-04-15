use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_umoka::UMokaPlugin;

fn main() {
    serve_plugin(&mut UMokaPlugin::new(), MsgPackSerializer {})
}
