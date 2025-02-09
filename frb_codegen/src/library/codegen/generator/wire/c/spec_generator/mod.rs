use crate::codegen::generator::misc::PathTexts;
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::misc::GeneratorProgressBarPack;
use serde::Serialize;

pub(super) mod cbindgen_executor;
pub(super) mod dummy_function;

#[derive(Clone, Serialize)]
pub(super) struct WireCOutputSpec {
    pub code_cbindgen: String,
    pub code_dummy: String,
}

pub(super) fn generate(
    config: &GeneratorWireCInternalConfig,
    extern_func_names: Vec<String>,
    extern_struct_names: Vec<String>,
    rust_output_texts: &PathTexts,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<WireCOutputSpec> {
    let code_cbindgen = cbindgen_executor::execute(
        config,
        extern_struct_names,
        rust_output_texts,
        progress_bar_pack,
    )?;
    let code_dummy = dummy_function::generate(extern_func_names);
    Ok(WireCOutputSpec {
        code_cbindgen,
        code_dummy,
    })
}
