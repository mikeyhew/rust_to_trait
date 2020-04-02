# `to_trait`

Provides the `To` trait, which provides methods similar to `.into()` and `.try_into()`, except they take type arguments. The trait looks like this:

```
trait To {
    fn to<T>(self) -> T where Self: Into<T>;
    fn try_to<T>(self) -> Result<T, Self::Error> where Self: TryInto<T>;
}
```

and you use it like this:

```
use to_trait::To;
let five_u64 = 5u32.to::<u64>();
let five_u8 = 5u32.to::<u8>().unwrap();
```

At first glance, this might not seem very useful, but sometimes the compiler can't infer the output type of a call to `.into()`. This happens a lot with method chains, e.g. if you call `.into().some_other_method()`, and it can be pretty annoying.

What woud be ideal is if we could write the desired output type as type arguments to the method. Unfortunately, because `Into<T>` takes `T` as a trait type parameter, we can't supply it when calling the method - the only way to supply the type arguments is by falling back to universal function call syntax, with `Into::<T>::into(..)`.

The methods on `To` trait essentially act as type inference helpers, letting you supply the type arguments while still using method call syntax. As a bonus, they are shorter by two characters :smile:

# License

MIT
