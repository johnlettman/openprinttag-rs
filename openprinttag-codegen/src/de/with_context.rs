
use serde::{de::DeserializeSeed, Deserializer};
use std::{collections::HashMap, marker::PhantomData, sync::Arc};
use crate::schema::context::LocalContext;

pub struct WithContext<'a, O>(pub &'a LocalContext<'a>, PhantomData<O>);

impl<'a, O> WithContext<'a, O> {
    pub fn new(local_context: &'a LocalContext<'a>) -> Self {
        Self(local_context, PhantomData)
    }
}

pub trait DeserializeWithContext<'a>: Sized {
    fn deserialize_with_context<'de, D>(
        deserializer: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

impl<'a, 'de, O> DeserializeSeed<'de> for WithContext<'a, O>
where
    O: DeserializeWithContext<'a>,
{
    type Value = O;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        O::deserialize_with_context(deserializer, self.0)
    }
}
