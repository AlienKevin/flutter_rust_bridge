use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::generator::misc::StructOrRecord;
use crate::codegen::generator::misc::StructOrRecord::Struct;
use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;
use itertools::Itertools;

impl<'a> CodecSseTyTrait for StructRefCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(self.new_generalized_generator().generate_encode(lang))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(
            self.new_generalized_generator()
                .generate_decode(lang, None, true),
        )
    }
}

impl<'a> StructRefCodecSseTy<'a> {
    fn new_generalized_generator(&self) -> GeneralizedStructGenerator {
        GeneralizedStructGenerator::new(self.ir.get(self.context.ir_pack).clone(), Struct)
    }
}

pub(crate) struct GeneralizedStructGenerator {
    st: IrStruct,
    mode: StructOrRecord,
}

impl GeneralizedStructGenerator {
    pub(crate) fn new(st: IrStruct, mode: StructOrRecord) -> Self {
        Self { st, mode }
    }

    pub(super) fn generate_encode(&self, lang: &Lang) -> String {
        (self.st.fields.iter().enumerate())
            .map(|(index, field)| {
                format!(
                    "{};\n",
                    lang.call_encode(
                        &field.ty,
                        &format!(
                            "self.{}",
                            self.mode
                                .field_name(index, field, self.st.is_fields_named, lang)
                        )
                    )
                )
            })
            .join("")
    }

    pub(super) fn generate_decode(
        &self,
        lang: &Lang,
        override_struct_name: Option<String>,
        dart_unconditionally_kwargs_ctor: bool,
    ) -> String {
        let decode_fields = (self.st.fields.iter().enumerate())
            .map(|(_index, field)| {
                format!(
                    "{} var_{} = {};\n",
                    lang.var_decl(),
                    field.name.dart_style(),
                    lang.call_decode(&field.ty)
                )
            })
            .join("");

        let ctor = match self.mode {
            Struct => lang.call_constructor(
                &override_struct_name.unwrap_or_else(|| self.st.name.style(lang)),
                &(self.st.fields.iter())
                    .map(|x| x.name.style(lang))
                    .collect_vec(),
                &(self.st.fields.iter())
                    .map(|x| format!("var_{}", x.name.dart_style().clone()))
                    .collect_vec(),
                (matches!(lang, Lang::DartLang(_)) && dart_unconditionally_kwargs_ctor)
                    || self.st.is_fields_named,
            ),
            StructOrRecord::Record => format!(
                "({})",
                (self.st.fields.iter())
                    .map(|x| format!("var_{}", x.name.dart_style().clone()))
                    .join(", ")
            ),
        };

        format!("{decode_fields}return {ctor};")
    }
}
