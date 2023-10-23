use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct DummySerializable<T>(pub T);

impl<T> Serialize for DummySerializable<T> {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        unreachable!()
    }
}

impl<T: Eq> Eq for DummySerializable<T> {}
impl<T: Copy> Copy for DummySerializable<T> {}
impl<T: Default> Default for DummySerializable<T> {
    fn default() -> Self {
        T::default().into()
    }
}

impl<T> From<T> for DummySerializable<T> {
    fn from(v: T) -> Self {
        DummySerializable(v)
    }
}
