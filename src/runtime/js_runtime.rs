use deno_core::{op, JsRuntime, RuntimeOptions};
use std::path::PathBuf;

pub fn start_js_runtime(root_path: PathBuf) -> JsRuntime {
    let mut options = RuntimeOptions::default();
    options.extensions.push("js".to_string());
    options.extensions.push("mjs".to_string());
    options.extensions.push("cjs".to_string());
    options.extensions.push("jsx".to_string());
    options.extensions.push("ts".to_string());
    options.extensions.push("tsx".to_string());
    options.extensions.push("json".to_string());
    options.allow_net = true;
    options.allow_read = true;
    options.allow_write = true;
    options.allow_env = true;
    options.allow_plugin = true;
    options.allow_hrtime = true;
    options.no_check_global = true;
    let js_runtime = JsRuntime::new(options);
    js_runtime.op(op!("op:readFile", |path: String| {
        let path = PathBuf::from(path);
        let contents = std::fs::read_to_string(path).unwrap();
        Ok(contents)
    }));
    js_runtime
}
