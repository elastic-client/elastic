//! Implementation of the Elasticsearch `geo_point` type.
//!
//! Geo points are an Elasticsearch specific geospatial type with an `x` (`lon`) and `y` (`lat`)
//! component.
//! `GeoPoint` is a good choice for storing and analysing geospatial points where geojson
//! compatibility isn't needed.
//!
//! # Examples
//!
//! For defining your own geo point mapping, see [mapping details](mapping/trait.GeoPointMapping.html#derive-mapping).
//!
//! Map with a default `geo_point`:
//!
//! ```
//! # use elastic_types::geo::point::prelude::*;
//! struct MyType {
//!     pub field: GeoPoint<DefaultGeoPointFormat>
//! }
//! ```
//!
//! Map with a custom `geo_point`:
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_derive)]
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # use std::marker::PhantomData;
//! # use elastic_types::prelude::*;
//! # fn main() {
//! # use elastic_types::prelude::*;
//! # use elastic_types::geo::point::prelude::*;
//! # #[derive(Default)]
//! # struct MyGeoPointMapping;
//! # impl GeoPointMapping for MyGeoPointMapping { type Format = GeoPointString; }
//! struct MyType {
//!     pub field: GeoPoint<GeoPointString, MyGeoPointMapping>
//! }
//! # }
//! ```
//!
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)

use georust::{Coordinate as C, Point as P, Geometry as G};

type Coordinate = C<f64>;
type Point = P<f64>;
type Geometry = G<f64>;

#[macro_use]
pub mod mapping;

mod point;
mod format;
mod formats;

pub use self::point::*;
pub use self::format::*;
pub use self::formats::*;

/// The default `geo_point` format (`GeoPointArray`).
pub type DefaultGeoPointFormat = GeoPointArray;

pub mod prelude {
    //! Includes all types for the `geo_point` type.
    //!
    //! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

    pub use super::DefaultGeoPointFormat;
    pub use super::format::*;
    pub use super::point::*;
    pub use super::formats::*;
    pub use super::mapping::*;
}
