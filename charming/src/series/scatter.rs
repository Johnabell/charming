use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        ColorBy, CoordinateSystem, DimensionEncode, Emphasis, ItemStyle, Label, MarkArea, MarkLine,
        Symbol, SymbolSize, Tooltip,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Scatter {
    #[serde(rename = "type")]
    #[charming_type = "scatter"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color: Option<String>,
    color_by: Option<ColorBy>,
    label: Option<Label>,
    dataset_index: Option<f64>,
    coordinate_system: Option<CoordinateSystem>,
    x_axis_index: Option<f64>,
    y_axis_index: Option<f64>,
    symbol: Option<Symbol>,
    symbol_size: Option<SymbolSize>,
    show_symbol: Option<bool>,
    large: Option<bool>,
    progressive: Option<f64>,
    zlevel: Option<f64>,
    encode: Option<DimensionEncode>,
    mark_line: Option<MarkLine>,
    mark_area: Option<MarkArea>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
