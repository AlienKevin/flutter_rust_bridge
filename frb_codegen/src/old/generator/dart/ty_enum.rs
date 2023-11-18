use itertools::Itertools;

use crate::generator::dart::dart_comments;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;
use crate::utils::misc::dart_maybe_implements_exception;

type_dart_generator_struct!(TypeEnumRefGenerator, IrTypeEnumRef);

impl TypeDartGeneratorTrait for TypeEnumRefGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let variants = (self.ir.get(self.context.ir_pack).variants())
            .iter()
            .enumerate()
            .map(|(idx, variant)| {
                let fields = match &variant.kind {
                    IrVariantKind::Value => vec![],
                    IrVariantKind::Struct(st) => (st.fields)
                        .iter()
                        .map(|field| {
                            format!(
                                ",api2wire_{}(raw.{})",
                                field.ty.safe_ident(),
                                field.name.dart_style()
                            )
                        })
                        .collect(),
                }
                .join("");
                format!(
                    "if (raw is {variant}) {{
                        return [{} {}];
                    }}",
                    idx,
                    fields,
                    variant = variant.wrapper_name.rust_style(),
                )
            })
            .join("\n");

        Acc {
            wasm: Some(format!(
                "{variants}

                throw Exception('unreachable');"
            )),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some(
            self.ir
                .get(self.context.ir_pack)
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    if let IrVariantKind::Value = &variant.kind {
                        format!(
                            "if (apiObj is {}) {{ wireObj.tag = {}; return; }}",
                            variant.wrapper_name, idx
                        )
                    } else {
                        let pre_field = match &variant.kind {
                            IrVariantKind::Struct(st) => st
                                .fields
                                .iter()
                                .map(|field| {
                                    format!(
                                        "var pre_{} = api2wire_{}(apiObj.{});",
                                        field.name.rust_style(),
                                        field.ty.safe_ident(),
                                        field.name.dart_style()
                                    )
                                })
                                .collect_vec(),
                            _ => unreachable!(),
                        };
                        let r = format!("wireObj.kind.ref.{}.ref", variant.name);
                        let body = match &variant.kind {
                            IrVariantKind::Struct(st) => st
                                .fields
                                .iter()
                                .map(|field| {
                                    format!(
                                        "{}.{name} = pre_{name};",
                                        r,
                                        name = field.name.rust_style(),
                                    )
                                })
                                .collect_vec(),
                            _ => unreachable!(),
                        };
                        format!(
                            "if (apiObj is {5}) {{
                                {3}
                                wireObj.tag = {1};
                                wireObj.kind = inner.inflate_{2}_{0}();
                                {4}
                                return;
                            }}",
                            variant.name,
                            idx,
                            self.ir.name,
                            pre_field.join("\n"),
                            body.join("\n"),
                            variant.wrapper_name
                        )
                    }
                })
                .join("\n"),
        )
    }
}
