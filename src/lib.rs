use std::convert::TryInto;

/// Provides methods similar to `.into()` and `.try_into()`,
/// except they take type arguments. See each method's documentation
/// for details.
///
pub trait To {
    /// A version of `std::convert::Into::into` that lets you provide
    /// type arguments to the method call.
    ///
    /// ```
    /// use to_trait::To;
    /// assert_eq!(5i32, 5u8.to::<i32>());
    /// ```
    fn to<T>(self) -> T
    where
        Self: Into<T>,
    {
        self.into()
    }

    /// a version of `std::convert::TryInto::try_into` that lets you provide type arguments
    /// to the method call.
    ///
    /// ```
    /// use to_trait::To;
    /// assert_eq!(255u32.try_to::<u8>(), Ok(255u8));
    /// assert!(256u32.try_to::<u8>().is_err());
    /// ```
    fn try_to<T>(self) -> Result<T, <Self as TryInto<T>>::Error>
    where
        Self: TryInto<T>,
    {
        self.try_into()
    }
}

impl<T> To for T {}

#[test]
fn test_it() {
    assert_eq!(42u32.to::<u64>(), 42u64);
    assert_eq!(42u32.try_to::<u8>(), Ok(42u8));
    assert!(500.try_to::<u8>().is_err());
}
