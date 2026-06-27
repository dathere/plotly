use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::color::Color;
use crate::layout::{Axis, Center, Projection};

/// Determines how a `geo` subplot's view is auto-computed to fit the plotted
/// data. The default is [`GeoFitBounds::False`] (equivalent to Plotly's
/// `false`), which uses the configured center/projection/axes as-is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GeoFitBounds {
    /// Equivalent to Plotly's `false`: use the configured
    /// center/projection/axes as-is. Serializes as the boolean `false`, so it
    /// can override a non-default value coming from a template or a previously
    /// composed layout.
    False,
    /// Frame the subplot to the union of the traces' location geometries.
    Locations,
    /// Frame the subplot to the bounds of the traces' GeoJSON.
    GeoJson,
}

impl Serialize for GeoFitBounds {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::False => serializer.serialize_bool(false),
            Self::Locations => serializer.serialize_str("locations"),
            Self::GeoJson => serializer.serialize_str("geojson"),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct LayoutGeo {
    /// Sets the latitude and longitude of the center of the map.
    center: Option<Center>,
    /// Sets the domain within which the mapbox will be drawn.
    /// Sets the zoom level of the map.
    zoom: Option<u8>,
    /// Sets the projection of the map
    #[field_setter(default = "Projection::new().projection_type(ProjectionType::Orthographic)")]
    projection: Option<Projection>,
    /// If to show the ocean or not
    #[field_setter(default = "Some(true)")]
    showocean: Option<bool>,
    /// Sets the color of the ocean
    #[field_setter(default = "'rgb(0, 255, 255)'")]
    oceancolor: Option<Box<dyn Color>>,
    /// If to show the land or not
    showland: Option<bool>,
    /// Sets the color of the land
    landcolor: Option<Box<dyn Color>>,
    /// If to show lakes or not
    showlakes: Option<bool>,
    /// Sets the color of the lakes
    lakecolor: Option<Box<dyn Color>>,
    /// If to show countries (borders) or not
    showcountries: Option<bool>,
    /// Configures the longitude axis
    lonaxis: Option<Axis>,
    /// Configures the latitude axis
    lataxis: Option<Axis>,
    /// Auto-frames the subplot to the plotted data (e.g. `Locations` fits the
    /// view to the trace's filled region geometries). Overrides
    /// `center`/`lonaxis`/`lataxis` ranges when set.
    #[serde(rename = "fitbounds")]
    fitbounds: Option<GeoFitBounds>,
    // Sets the coastline stroke width (in px).
    #[field_setter(default = "Some(1)")]
    coastlinewidth: Option<u8>,
}

impl LayoutGeo {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::color::Rgb;
    use crate::layout::{Axis, Center, Projection, ProjectionType, Rotation};

    #[test]
    fn serialize_layout_geo() {
        let geo = LayoutGeo::new()
            .center(Center::new(10.0, 20.0))
            .zoom(5)
            .projection(
                Projection::new()
                    .projection_type(ProjectionType::Mercator)
                    .rotation(Rotation::new().lat(1.0).lon(2.0).roll(4.0)),
            )
            .showocean(true)
            .oceancolor(Rgb::new(0, 255, 255))
            .showland(true)
            .landcolor(Rgb::new(100, 200, 100))
            .showlakes(false)
            .lakecolor(Rgb::new(50, 50, 200))
            .showcountries(true)
            .lonaxis(Axis::new().title("Longitude"))
            .lataxis(Axis::new().title("Latitude"))
            .coastlinewidth(2);

        let expected = json!({
            "center": {"lat": 10.0, "lon": 20.0},
            "zoom": 5,
            "projection": {"type": "mercator", "rotation": {"lat": 1.0, "lon": 2.0, "roll": 4.0}},
            "showocean": true,
            "oceancolor": "rgb(0, 255, 255)",
            "showland": true,
            "landcolor": "rgb(100, 200, 100)",
            "showlakes": false,
            "lakecolor": "rgb(50, 50, 200)",
            "showcountries": true,
            "lataxis": { "title": { "text": "Latitude" } },
            "lonaxis": { "title": { "text": "Longitude" } },
            "coastlinewidth": 2
        });
        assert_eq!(to_value(geo).unwrap(), expected);
    }

    #[test]
    fn serialize_geo_fitbounds() {
        let geo = LayoutGeo::new().fitbounds(GeoFitBounds::Locations);
        assert_eq!(to_value(geo).unwrap(), json!({ "fitbounds": "locations" }));
        assert_eq!(to_value(GeoFitBounds::GeoJson).unwrap(), json!("geojson"));
        assert_eq!(to_value(GeoFitBounds::False).unwrap(), json!(false));

        let geo = LayoutGeo::new().fitbounds(GeoFitBounds::False);
        assert_eq!(to_value(geo).unwrap(), json!({ "fitbounds": false }));
    }
}
