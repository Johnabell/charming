use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint, Dimension},
    element::{
        ColorBy, CoordinateSystem, DimensionEncode, ItemStyle, LabelLayout, LabelLine, RawString,
        Tooltip,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{DisplayFromStr, PickFirst, serde_as};

/// TODO remove this since the json that requires this is actaully incorrect. But needs an upstream
/// fix
fn deserialize_opt_clip<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum ClipRaw {
        Bool(bool),
        Str(String),
    }
    match Option::<ClipRaw>::deserialize(deserializer)? {
        None => Ok(None),
        Some(ClipRaw::Bool(b)) => Ok(Some(b)),
        Some(ClipRaw::Str(s)) => Ok(Some(
            matches!(s.as_str(), "true" | "True" | "1") || s.eq_ignore_ascii_case("yes"),
        )),
    }
}

#[serde_as]
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    #[serde(rename = "type")]
    #[charming_type = "custom"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    legend_hover_link: Option<bool>,
    coordinate_system: Option<CoordinateSystem>,
    x_axis_index: Option<CompositeValue>,
    y_axis_index: Option<CompositeValue>,
    polar_index: Option<CompositeValue>,
    geo_index: Option<CompositeValue>,
    calendar_index: Option<CompositeValue>,
    render_item: Option<RawString>,
    item_style: Option<ItemStyle>,
    label_line: Option<LabelLine>,
    label_layout: Option<LabelLayout>,
    selected_mode: Option<bool>,
    zlevel: Option<f64>,
    silent: Option<bool>,
    #[serde_with(skip)]
    #[serde(default, deserialize_with = "deserialize_opt_clip")]
    clip: Option<bool>,
    #[charming_set_vec]
    #[serde_as(as = "Vec<PickFirst<(_, DisplayFromStr)>>")]
    dimensions: Vec<Dimension>,
    encode: Option<DimensionEncode>,
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
