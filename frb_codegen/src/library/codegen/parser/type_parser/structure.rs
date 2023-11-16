use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::structure::IrStruct;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::TypeParser;
use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, Ident};

impl<'a> TypeParser<'a> {
    pub(crate) fn parse_struct(&mut self, ident_string: &str) -> anyhow::Result<Option<IrStruct>> {
        let src_struct = self.src_structs[ident_string];

        let (is_fields_named, struct_fields) = match &src_struct.0.src.fields {
            Fields::Named(FieldsNamed { named, .. }) => (true, named),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (false, unnamed),
            _ => return Ok(None),
        };

        let fields = struct_fields
            .iter()
            .enumerate()
            .map(|(idx, field)| self.parse_struct_field(idx, field))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let (name, wrapper_name) =
            compute_name_and_wrapper_name(&src_struct.0.ident, src_struct.0.mirror);

        let path = Some(src_struct.0.path.clone());
        let comments = parse_comments(&src_struct.0.src.attrs);

        let attributes = FrbAttributes::parse(&src_struct.0.src.attrs)?;
        let dart_metadata = attributes.dart_metadata();

        Ok(Some(IrStruct {
            name,
            wrapper_name,
            path,
            fields,
            is_fields_named,
            dart_metadata,
            comments,
        }))
    }

    fn parse_struct_field(&mut self, idx: usize, field: &Field) -> anyhow::Result<IrField> {
        let field_name = field
            .ident
            .as_ref()
            .map_or(format!("field{idx}"), ToString::to_string);
        let field_type = self.parse_type(&field.ty)?;
        let attributes = FrbAttributes::parse(&field.attrs)?;
        Ok(IrField {
            name: IrIdent::new(field_name),
            ty: field_type,
            is_final: !attributes.non_final(),
            comments: parse_comments(&field.attrs),
            default: attributes.default_value(),
            settings: IrFieldSettings::default(),
        })
    }
}

pub(super) fn compute_name_and_wrapper_name(
    ident: &Ident,
    mirror: bool,
) -> (String, Option<String>) {
    let name = ident.to_string();
    let wrapper_name = if mirror {
        Some(format!("mirror_{name}"))
    } else {
        None
    };
    (name, wrapper_name)
}
