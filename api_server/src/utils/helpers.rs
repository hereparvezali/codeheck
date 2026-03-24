use sea_orm::{
    ActiveValue::{self, NotSet, Set},
    Value,
};

pub trait SetFromValue {
    type Output;
    fn set(self) -> ActiveValue<Self::Output>
    where
        Self::Output: Into<Value>;
}
impl<T> SetFromValue for T
where
    T: Into<Value>,
{
    type Output = T;
    fn set(self) -> ActiveValue<Self::Output>
    where
        Self::Output: Into<Value>,
    {
        Set(self)
    }
}
pub trait SetFromOption {
    type Output;
    fn set_from_opt(self) -> ActiveValue<Self::Output>
    where
        Self::Output: Into<Value>;
}
impl<T> SetFromOption for Option<T>
where
    T: Into<Value>,
{
    type Output = T;

    fn set_from_opt(self) -> ActiveValue<Self::Output> {
        match self {
            Some(v) => Set(v),
            None => NotSet,
        }
    }
}
