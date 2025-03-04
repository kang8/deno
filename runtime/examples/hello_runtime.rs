// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

fn get_error_class_name(e: &AnyError) -> &'static str {
  deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
  let module_loader = Rc::new(FsModuleLoader);
  let create_web_worker_cb = Arc::new(|_| {
    todo!("Web workers are not supported in the example");
  });
  let web_worker_event_cb = Arc::new(|_| {
    todo!("Web workers are not supported in the example");
  });

  let options = WorkerOptions {
    bootstrap: BootstrapOptions::default(),
    extensions: vec![],
    startup_snapshot: None,
    unsafely_ignore_certificate_errors: None,
    root_cert_store_provider: None,
    seed: None,
    source_map_getter: None,
    format_js_error_fn: None,
    web_worker_preload_module_cb: web_worker_event_cb.clone(),
    web_worker_pre_execute_module_cb: web_worker_event_cb,
    create_web_worker_cb,
    maybe_inspector_server: None,
    should_break_on_first_statement: false,
    should_wait_for_inspector_session: false,
    module_loader,
    node_fs: None,
    npm_resolver: None,
    get_error_class_fn: Some(&get_error_class_name),
    cache_storage_dir: None,
    origin_storage_dir: None,
    blob_store: BlobStore::default(),
    broadcast_channel: InMemoryBroadcastChannel::default(),
    shared_array_buffer_store: None,
    compiled_wasm_module_store: None,
    stdio: Default::default(),
  };

  let js_path =
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/hello_runtime.js");
  let main_module = deno_core::resolve_path(
    &js_path.to_string_lossy(),
    &std::env::current_dir().context("Unable to get CWD")?,
  )?;
  let permissions = PermissionsContainer::allow_all();

  let mut worker = MainWorker::bootstrap_from_options(
    main_module.clone(),
    permissions,
    options,
  );
  worker.execute_main_module(&main_module).await?;
  worker.run_event_loop(false).await?;
  Ok(())
}
