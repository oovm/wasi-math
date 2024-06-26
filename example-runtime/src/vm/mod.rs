use std::path::Path;

use wasmtime::{
    component::{Component, Instance, Linker, ResourceTable},
    Config, Engine, Store,
};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

use crate::{MathContext, wasi};

pub struct MathRuntime {
    store: Store<ContextView>,
    instance: Instance,
}

impl MathRuntime {
    pub async fn load_wast(wast: &str) -> anyhow::Result<Self> {
        let engine = get_engine()?;
        let binary = wat::parse_str(wast)?;
        let component = Component::new(&engine, binary.as_slice())?;
        get_component(engine, component).await
    }
    pub async fn load_wasm(wasm: &[u8]) -> anyhow::Result<Self> {
        let engine = get_engine()?;
        let component = Component::from_binary(&engine, wasm)?;
        get_component(engine, component).await
    }
    pub async fn load_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let engine = get_engine()?;
        let component = Component::from_file(&engine, path)?;
        get_component(engine, component).await
    }
}

fn get_engine() -> anyhow::Result<Engine> {
    let mut config = Config::new();
    {
        config.async_support(true);
        config.wasm_component_model(true);
    }
    {
        config.debug_info(true);
        config.wasm_backtrace(true);
    }
    {
        config.wasm_gc(true);
        config.wasm_function_references(true);
        config.wasm_reference_types(true);
        config.wasm_memory64(true);
    }
    Engine::new(&config)
}

async fn get_component(engine: Engine, input: Component) -> anyhow::Result<MathRuntime> {
    let mut store = {
        let mut builder = WasiCtxBuilder::new();
        builder.inherit_stderr();
        builder.inherit_stdout();
        builder.inherit_stdin();
        Store::new(&engine, ContextView::new(ResourceTable::default(), builder.build()))
    };
    let instance = {
        let mut linker = Linker::<ContextView>::new(&engine);
        linker.allow_shadowing(true);
        wasmtime_wasi::command::add_to_linker(&mut linker)?;
        wasi::math::types::add_to_linker(&mut linker, |state| &mut state.extension)?;
        wasi::math::conversion::add_to_linker(&mut linker, |state| &mut state.extension)?;
        wasi::math::arithmetic::add_to_linker(&mut linker, |state| &mut state.extension)?;
        linker.instantiate_async(&mut store, &input).await?
    };
    Ok(MathRuntime { store, instance })
}

pub struct ContextView {
    wasi: WasiCtx,
    extension: MathContext,
}

impl ContextView {
    fn new(table: ResourceTable, wasi: WasiCtx) -> Self {
        Self { wasi, extension: MathContext { table } }
    }
}

impl WasiView for ContextView {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.extension.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}
