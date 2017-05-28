use std::marker::PhantomData;
use serde::Serialize;

pub trait SerializeField<F> {
    type Field: Serialize + Default;
}

pub trait FieldMapping<F>
    where Self: Default + SerializeField<F>,
          F: Default
{
    fn data_type() -> &'static str {
        "object"
    }
}

/**
A wrapper type used to work around conflicting implementations of `Serialize`
for the various mapping traits.

Serialising `Field` will produce the mapping for the given type,
suitable as the mapping of a field for a document.
**/
#[derive(Default)]
pub struct DocumentField<M, F>
    where M: FieldMapping<F>,
          F: Default
{
    _m: PhantomData<(M, F)>,
}

impl<M, F> From<M> for DocumentField<M, F>
    where M: FieldMapping<F>,
          F: Default
{
    fn from(_: M) -> Self {
        DocumentField::<M, F>::default()
    }
}
