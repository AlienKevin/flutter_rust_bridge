use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::generator::wire::dart::wire2api::ty::WireDartGeneratorWire2apiTrait;

impl<'a> WireDartGeneratorWire2apiTrait for RecordWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        let len = self.ir.values.len();
        let values = self
            .ir
            .values
            .iter()
            .enumerate()
            .map(|(idx, ty)| format!("_wire2api_{}(arr[{idx}])", ty.safe_ident()))
            .collect_vec()
            .join(",");
        format!(
            "final arr = raw as List<dynamic>;
            if (arr.length != {len}) {{
                throw Exception('Expected {len} elements, got ${{arr.length}}');
            }}
            return ({values},);"
        )
    }
}
