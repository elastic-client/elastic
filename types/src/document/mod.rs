//! Base requirements for indexable document mappings.
//!
//! There are two kinds of types we can map in Elasticsearch; `field`/`data` types and `document` types:
//! 
//! - `FieldType` for types that can be mapped as fields on another type
//! - `DocumentType + FieldType` for types that can be indexed as documents.
//! 
//! Most of the work lives in the `FieldMapping`, which holds the serialisation requirements
//! to convert a Rust type into an Elasticsearch mapping.
//! Document types must also implement `DocumentMapping`, which maps the fields of a struct as properties,
//! and treats the type as `nested` when used as a field itself.
//! 
//! # Examples
//!
//! Define your Elasticsearch types using _Plain Old Rust Structures_.
//! Your types should at least derive `Default`, `Clone` and `serde::Serialize`.
//!
//! ## Derive Mapping
//!
//! Mapping can be generated by deriving `ElasticType` on a struct:
//!
//! ```
//! # #![feature(proc_macro)]
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use elastic_types::prelude::*;
//! #[derive(Serialize, ElasticType)]
//! pub struct MyType {
//!     pub my_date: Date<DefaultDateFormat>,
//!     pub my_string: String,
//!     pub my_num: i32
//! }
//! # fn main() {
//! # }
//! ```
//!
//! This will produce the following mapping:
//!
//! ```
//! # #![feature(proc_macro)]
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # extern crate serde_json;
//! # use elastic_types::prelude::*;
//! # #[derive(Serialize, ElasticType)]
//! # pub struct MyType {
//! #   pub my_date: Date<DefaultDateFormat>,
//! #   pub my_string: String,
//! #   pub my_num: i32
//! # }
//! # fn main() {
//! # let mapping = serde_json::to_string(&MyTypeMapping).unwrap();
//! # let json = json_str!(
//! {
//!     "type": "nested",
//!     "properties": {
//!         "my_date": {
//!             "type": "date",
//!             "format": "basic_date_time"
//!         },
//!         "my_string": {
//!             "type": "text",
//!             "fields": {
//!                 "keyword":{
//!                     "type":"keyword",
//!                     "ignore_above":256
//!                 }
//!             }
//!         },
//!         "my_num": {
//!             "type": "integer"
//!         }
//!     }
//! }
//! # );
//! # assert_eq!(json, mapping);
//! # }
//! ```
//!
//! It's also possible to adjust the mapping using the `#[elastic]` attribute.
//!
//! ### Override Default Mapping Properties
//!
//! You can override the mapping meta properties for an object by providing your own mapping type with `#[elastic(mapping="{TypeName}")]`:
//!
//! ```
//! # #![feature(proc_macro)]
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use elastic_types::prelude::*;
//! #[derive(Serialize, ElasticType)]
//! #[elastic(mapping="MyTypeMapping")]
//! pub struct MyType {
//!     pub my_date: Date<DefaultDateFormat>,
//!     pub my_string: String,
//!     pub my_num: i32
//! }
//!
//! #[derive(Default)]
//! struct MyTypeMapping;
//! impl DocumentMapping for MyTypeMapping {
//!     //Give your own name to a type
//!     fn name() -> &'static str { "my_type" }
//!
//!     fn data_type() -> &'static str { OBJECT_DATATYPE }
//! }
//! # fn main() {
//! # }
//! ```
//!
//! This will produce the following mapping:
//!
//! ```
//! # #![feature(proc_macro)]
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # extern crate serde_json;
//! # use elastic_types::prelude::*;
//! # #[derive(Default, Serialize, Deserialize, ElasticType)]
//! # #[elastic(mapping="MyTypeMapping")]
//! # pub struct MyType {
//! #   pub my_date: Date<DefaultDateFormat>,
//! #   pub my_string: String,
//! #   pub my_num: i32
//! # }
//! #
//! # #[derive(Default)]
//! # struct MyTypeMapping;
//! # impl DocumentMapping for MyTypeMapping {
//! #   fn name() -> &'static str { "my_type" }
//! #   fn data_type() -> &'static str { OBJECT_DATATYPE }
//! # }
//! # fn main() {
//! # let mapping = serde_json::to_string(&MyTypeMapping).unwrap();
//! # let json = json_str!(
//! {
//!     "type": "object",
//!     "properties": {
//!         "my_date": {
//!             "type": "date",
//!             "format": "basic_date_time"
//!         },
//!         "my_string": {
//!             "type": "text",
//!             "fields": {
//!                 "keyword":{
//!                     "type":"keyword",
//!                     "ignore_above":256
//!                 }
//!             }
//!         },
//!         "my_num": {
//!             "type": "integer"
//!         }
//!     }
//! }
//! # );
//! # assert_eq!(json, mapping);
//! # }
//! ```
//!
//! ### Ignore or Rename Fields
//!
//! You can then serialise type mappings with `#[serde]` attributes:
//!
//! ```
//! # #![feature(proc_macro)]
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use elastic_types::prelude::*;
//! #[derive(ElasticType, Serialize)]
//! pub struct MyType {
//!     #[serde(rename="my_renamed_date")]
//!     pub my_date: Date<DefaultDateFormat>,
//!     #[serde(skip_serializing)]
//!     pub ignored: String,
//!     pub my_num: i32
//! }
//! # fn main() {
//! # }
//! ```
//!
//! > NOTE: Fields with a `#[serde(skip_deserializing)]` attribute will still be mapped, because they can
//! still be indexed in Elasticsearch.
//!
//! ## Limitations
//!
//! Automatically deriving mapping has the following limitations:
//!
//! - Generics aren't supported by auto deriving.
//! So you can't `#[derive(ElasticType)]` on `MyType<T>`.
//! - Mapping types can't be shared. This is because they need to map the type fields, so are specific to that type.
//! So you can't share `MyTypeMapping` between `MyType` and `MyOtherType`.
//!
//! All of the above limitations can be worked around by implementing the mapping manually.
//!
//! Remember that Elasticsearch will automatically update mappings based on the objects it sees though,
//! so if your 'un-mapped' field is serialised on `index`, then some mapping will be added for it.
//!
//! ## Manually Implement Mapping
//!
//! You can build object mappings on `stable` by manually implementing the [`DocumentMapping`](trait.DocumentMapping.html) and [`PropertiesMapping`](trait.PropertiesMapping.html) traits:
//!
//! ```
//! # #![feature(proc_macro)]
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use elastic_types::prelude::*;
//! #[derive(Serialize)]
//! pub struct MyType {
//!     pub my_date: Date<DefaultDateFormat>,
//!     pub my_string: String,
//!     pub my_num: i32
//! }
//!
//! //Implement DocumentType for your type. This binds it to the mapping
//! impl DocumentType<MyTypeMapping> for MyType { }
//!
//! //Define the type mapping for our type
//! #[derive(Default)]
//! pub struct MyTypeMapping;
//! impl DocumentMapping for MyTypeMapping {
//!     fn name() -> &'static str { "my_type" }
//! }
//! impl PropertiesMapping for MyTypeMapping {
//!     fn props_len() -> usize { 3 }
//!
//!     fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error>
//!     where S: serde::Serializer {
//!         try!(field_ser(serializer, state, "my_date", Date::<DefaultDateFormat>::mapping()));
//!         try!(field_ser(serializer, state, "my_string", String::mapping()));
//!         try!(field_ser(serializer, state, "my_num", i32::mapping()));
//!
//!         Ok(())
//!     }
//! }
//! # fn main() {
//! # }
//! ```
//! 
//! ## Using the `Document` type
//! 
//! To serialise a document mapping, you can use its mapping type as a generic parameter in `Document<M>`.
//! For example, we can define an index type for the 
//! [Create Index API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html#mappings) 
//! that includes the mapping for `MyType`:
//! 
//! ```
//! # #![feature(proc_macro)]
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use elastic_types::prelude::*;
//! #[derive(Serialize, ElasticType)]
//! pub struct MyType {
//!     pub my_date: Date<DefaultDateFormat>,
//!     pub my_string: String,
//!     pub my_num: i32
//! }
//! 
//! #[derive(Default, Serialize)]
//! pub struct MyIndex {
//!     pub mappings: Mappings
//! }
//! 
//! #[derive(Default, Serialize)]
//! pub struct Mappings {
//!     pub mytype: Document<MyTypeMapping>
//! }
//! # fn main() {
//! # }
//! ```
//! 
//! Serialising `MyIndex` will produce the following json:
//!
//! ```
//! # #![feature(proc_macro)]
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # extern crate serde_json;
//! # use elastic_types::prelude::*;
//! # #[derive(Serialize, ElasticType)]
//! # pub struct MyType {
//! #     pub my_date: Date<DefaultDateFormat>,
//! #     pub my_string: String,
//! #     pub my_num: i32
//! # }
//! # #[derive(Default, Serialize)]
//! # pub struct MyIndex {
//! #     pub mappings: Mappings
//! # }
//! # #[derive(Default, Serialize)]
//! # pub struct Mappings {
//! #     pub mytype: Document<MyTypeMapping>
//! # }
//! # fn main() {
//! # let index = serde_json::to_string(&MyIndex::default()).unwrap();
//! # let json = json_str!(
//! {
//!     "mappings": {
//!         "mytype": {
//!             "properties": {
//!                 "my_date": {
//!                     "type": "date",
//!                     "format": "basic_date_time"
//!                 },
//!                 "my_string": {
//!                     "type": "text",
//!                     "fields": {
//!                         "keyword":{
//!                             "type":"keyword",
//!                             "ignore_above":256
//!                         }
//!                     }
//!                 },
//!                 "my_num": {
//!                     "type": "integer"
//!                 }
//!             }
//!         }
//!     }
//! }
//! # );
//! # assert_eq!(json, index);
//! # }
//! ```
//!
//! # Links
//! - [Field Types](https://www.elastic.co/guide/en/elasticsearch/reference/master/mapping-types.html)
//! - [Document Types](https://www.elastic.co/guide/en/elasticsearch/reference/master/mapping.html)

use std::marker::PhantomData;
use serde::{Serialize, Serializer};
use ::field::{FieldType, FieldMapping, Field};

/// The additional fields available to an indexable Elasticsearch type.
///
/// This trait is implemented for the type being mapped, rather than the mapping
/// type itself.
pub trait DocumentType<M>
    where M: DocumentMapping,
          Self: Serialize
{
    /// Get the name for this type.
    ///
    /// This is a convenience method that returns the `name` of the bound `DocumentMapping`.
    fn name() -> &'static str {
        M::name()
    }
}

impl<T, M> FieldType<M, DocumentFormat> for T
    where T: DocumentType<M>,
          M: DocumentMapping
{
}

/// A wrapper type for serialising user types.
#[derive(Default)]
pub struct Document<M>
    where M: DocumentMapping
{
    _m: PhantomData<M>,
}

impl<M> From<M> for Document<M>
    where M: DocumentMapping
{
    fn from(_: M) -> Self {
        Document::<M>::default()
    }
}

impl<M> Serialize for Document<M>
    where M: DocumentMapping
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 1));

        try!(serializer.serialize_struct_elt(&mut state, "properties", &Properties::<M>::default()));

        serializer.serialize_struct_end(state)
    }
}

/// Elasticsearch datatype name.
pub const OBJECT_DATATYPE: &'static str = "object";
/// Elasticsearch datatype name.
pub const DYNAMIC_DATATYPE: &'static str = "dynamic";
/// Elasticsearch datatype name.
pub const NESTED_DATATYPE: &'static str = "nested";

#[doc(hidden)]
#[derive(Default)]
pub struct DocumentFormat;

/// The base requirements for mapping an `object` type.
pub trait DocumentMapping
    where Self: PropertiesMapping + Default
{
    /// Get the indexed name for this mapping.
    fn name() -> &'static str;

    /// Get the type name for this mapping, like `object` or `nested`.
    fn data_type() -> &'static str {
        NESTED_DATATYPE
    }

    /// Whether or not new properties should be added dynamically to an existing object.
    /// Accepts `true` (default), `false` and `strict`.
    fn dynamic() -> Option<Dynamic> {
        None
    }

    /// Whether the JSON value given for the object field should be parsed and indexed
    /// (`true`, default) or completely ignored (`false`).
    fn enabled() -> Option<bool> {
        None
    }

    /// Sets the default `include_in_all` value for all the properties within the object.
    /// The object itself is not added to the `_all` field.
    fn include_in_all() -> Option<bool> {
        None
    }
}

/// Serialisation for the mapping of object properties.
///
/// This trait is designed to be auto-derived, so it expects you to be familiar with how `serde` works.
///
/// # Examples
///
/// Say we have a mappable type with 3 fields called `MyType` and a mapping type called `MyTypeMapping`:
///
/// ```
/// # use elastic_types::prelude::*;
/// struct MyType {
///     pub my_date: Date<DefaultDateFormat>,
///     pub my_string: String,
///     pub my_num: i32
/// }
///
/// #[derive(Default)]
/// struct MyTypeMapping;
/// ```
///
/// To serialise the mapping of each of `MyType`s fields, we implement `PropertiesMapping` for `MyTypeMapping`,
/// and use `serde` to serialise the mapping types for each field.
///
/// ```
/// # #![feature(proc_macro)]
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # #[macro_use]
/// # extern crate elastic_types_derive;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # use elastic_types::prelude::*;
/// # pub struct MyTypeMapping;
/// impl PropertiesMapping for MyTypeMapping {
///     fn props_len() -> usize { 3 }
///
///     fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error>
///     where S: serde::Serializer {
///         try!(field_ser(serializer, state, "my_date", Date::<DefaultDateFormat>::mapping()));
///         try!(field_ser(serializer, state, "my_string", String::mapping()));
///         try!(field_ser(serializer, state, "my_num", i32::mapping()));
///
///         Ok(())
///     }
/// }
/// # fn main() {
/// # }
/// ```
///
/// It's easy to get an instance of the mapping for a given type by calling the static `mapping` function.
/// This trait is automatically implemented for you when you `#[derive(ElasticType)]`.
pub trait PropertiesMapping {
    /// The number of mapped property fields for this type.
    ///
    /// This number should be the same as the number of fields being serialised by `serialize_props`.
    fn props_len() -> usize;

    /// Serialisation for the mapped property fields on this type.
    ///
    /// You can use the `field_ser!` macro to simplify `serde` calls.
    fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error> where S: Serializer;
}

impl<T> FieldMapping<DocumentFormat> for T
    where T: DocumentMapping
{
    type Field = Field<T, DocumentFormat>;

    fn data_type() -> &'static str {
        <Self as DocumentMapping>::data_type()
    }
}

impl<T> Serialize for Field<T, DocumentFormat>
    where T: FieldMapping<DocumentFormat> + DocumentMapping
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 5));

        let ty = <T as DocumentMapping>::data_type();
        try!(serializer.serialize_struct_elt(&mut state, "type", ty));

        ser_field!(serializer, &mut state, "dynamic", T::dynamic());
        ser_field!(serializer,
                   &mut state,
                   "include_in_all",
                   T::include_in_all());

        if ty == OBJECT_DATATYPE {
            ser_field!(serializer, &mut state, "enabled", T::enabled());
        }

        try!(serializer.serialize_struct_elt(&mut state, "properties", &Properties::<T>::default()));

        serializer.serialize_struct_end(state)
    }
}

#[derive(Default)]
struct Properties<M>
    where M: DocumentMapping
{
    _m: PhantomData<M>,
}

impl<M> Serialize for Properties<M>
    where M: DocumentMapping
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("properties", M::props_len()));
        try!(M::serialize_props(serializer, &mut state));
        serializer.serialize_struct_end(state)
    }
}

/// The dynamic setting may be set at the mapping type level, and on each inner object.
/// Inner objects inherit the setting from their parent object or from the mapping type.
#[derive(Debug, Clone, Copy)]
pub enum Dynamic {
    /// Newly detected fields are added to the mapping. (default).
    True,
    /// Newly detected fields are ignored. New fields must be added explicitly.
    False,
    /// If new fields are detected, an exception is thrown and the document is rejected.
    Strict,
}

impl Serialize for Dynamic {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        match *self {
            Dynamic::True => serializer.serialize_bool(true),
            Dynamic::False => serializer.serialize_bool(false),
            Dynamic::Strict => serializer.serialize_str("strict"),
        }
    }
}

/// Serialise a field mapping using the given serialiser.
#[inline]
pub fn field_ser<S, M, F>(serializer: &mut S, state: &mut S::StructState, field: &'static str, _: M) -> Result<(), S::Error>
    where S: Serializer,
          M: FieldMapping<F>,
          F: Default
{
    serializer.serialize_struct_elt(state, field, &M::Field::default())
}

/// Serialise a document mapping using the given serialiser.
#[inline]
pub fn doc_ser<S, M>(serializer: &mut S, state: &mut S::StructState, field: &'static str, _: M) -> Result<(), S::Error>
    where S: Serializer,
          M: DocumentMapping
{
    serializer.serialize_struct_elt(state, field, &Document::<M>::default())
}