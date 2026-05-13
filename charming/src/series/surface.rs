use crate::element::{ItemStyle, Tooltip};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParametricAxisRange {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParametricEquation {
    pub u: ParametricAxisRange,
    pub v: ParametricAxisRange,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Wireframe {
    pub show: Option<bool>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Surface {
    #[serde(rename = "type")]
    #[charming_type = "surface"]
    type_: String,
    color: Option<String>,
    grid3d_index: Option<f64>,
    item_style: Option<ItemStyle>,
    parametric: Option<bool>,
    parametric_equation: Option<ParametricEquation>,
    tooltip: Option<Tooltip>,
    wireframe: Option<Wireframe>,
}
