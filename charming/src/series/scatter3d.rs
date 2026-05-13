use crate::{
    datatype::{DataFrame, DataPoint},
    element::{DimensionEncode, Label, Symbol, SymbolSize},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Scatter3d {
    #[serde(rename = "type")]
    #[charming_type = "scatter3D"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    grid3d_index: Option<f64>,
    encode: Option<DimensionEncode>,
    label: Option<Label>,
    symbol: Option<Symbol>,
    symbol_size: Option<SymbolSize>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
