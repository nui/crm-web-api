pub mod support;

/// Combine multiple `warp::Filter` using `.or`
///
/// Note: Debug build box all filters to avoid Tokio stack overflow at runtime.
/// See https://github.com/seanmonstar/warp/issues/811
#[cfg(build_profile = "debug")]
macro_rules! combine {
    ($x:expr $(,)?) => (
        $x.boxed()
    );
    ($x0:expr, $($x:expr),+ $(,)?) => (
        $x0.boxed()$(.or($x.boxed()))+.boxed()
    );
}
#[cfg(build_profile = "release")]
macro_rules! combine {
    ($x:expr $(,)?) => (
        $x
    );
    ($x0:expr, $($x:expr),+ $(,)?) => (
        $x0$(.or($x))+
    );
}

/// Measure given expression usage time
///
/// If expression is early return, Time will not be measured.
/// It occur when we use `?` in expression.
/// Some kind of that expression can be measured by moving `?` out of expression.
///
/// For example
///
/// This is async method that return `Result`
/// ```
/// async fn get() -> Result<(), ()> {}
/// ```
/// Change this
///     `measure_time!("TAG", get().await?)`
/// To this
///     `measure_time!("TAG", get().await)?`
/// Will measure time correctly.
///
/// NOTE:
///     - Use `;` as a unit separator cause rustfmt at call site not working properly.
///     - `$tag` can be anything that implement `std::fmt::Display`
macro_rules! measure_time {
    // We usually use this variant
    ($tag:expr, $expr:expr) => { measure_time!(AUTO, $tag, $expr) };
    // Auto unit implementation
    (AUTO, $tag:expr, $expr:expr) => {
        {
            let start = ::std::time::Instant::now();
            let value = $expr;
            ::tracing::debug!(
                "{} in {}",
                $tag,
                crate::_macros::support::DisplayWithAutoUnit::from(start),
            );
            value
        }
    };
    // Use following variants when custom unit is desire
    (MILLI, $tag:expr, $expr:expr) => { measure_time!(["ms", as_millis]; $tag, $expr) };
    (MICRO, $tag:expr, $expr:expr) => { measure_time!(["µs", as_micros]; $tag, $expr) };
    (NANO,  $tag:expr, $expr:expr) => { measure_time!(["ns", as_nanos];  $tag, $expr) };
    (SEC,   $tag:expr, $expr:expr) => { measure_time!(["s",  as_secs];   $tag, $expr) };
    // Custom unit implementation
    ([$unit:literal, $as_unit:ident]; $tag:expr, $expr:expr) => {
        {
            let start = ::std::time::Instant::now();
            let value = $expr;
            ::tracing::debug!(
                ::core::concat!("{} in {} ", $unit),
                $tag,
                start.elapsed().$as_unit(),
            );
            value
        }
    };
}
