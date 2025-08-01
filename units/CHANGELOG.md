# 1.0.0 - 2025-02-24

BOOM! A long time in the making but here goes, our first 1.0 crate release.

This changelog is a rolling description of everything that will eventually end up in `v1.0`.

* Introduce limit to `Amount`
  * Prepare to enforce `MAX_MONEY` invariant [#4164](https://github.com/rust-bitcoin/rust-bitcoin/pull/4164)
  * Enforce `MAX_MONEY` invariant in amount types [#4157](https://github.com/rust-bitcoin/rust-bitcoin/pull/4157)
* New `NumOpResult` type
  * Introduce monadic `NumOpResult` type [#4007](https://github.com/rust-bitcoin/rust-bitcoin/pull/4007)
  * Add impls for `NumOpResult` div and mul [#4337](https://github.com/rust-bitcoin/rust-bitcoin/pull/4337)
  * Use `NumOpResult` instead of `Option` [#4428](https://github.com/rust-bitcoin/rust-bitcoin/pull/4428)
  * Return `NumOpResult` when implementing `Div` [#4312](https://github.com/rust-bitcoin/rust-bitcoin/pull/4312)
* Heavily modify `fee_rate` module:
  * Make `FeeRate` use MvB internally [#4534](https://github.com/rust-bitcoin/rust-bitcoin/pull/4534)
  * Add `FeeRate` addition and subtraction traits [#3381](https://github.com/rust-bitcoin/rust-bitcoin/pull/3381)
  * Remove `Display`/`FromStr` for `FeeRate` [#4512](https://github.com/rust-bitcoin/rust-bitcoin/pull/4512)
  * Implement `serde` modules for `FeeRate` [#3666](https://github.com/rust-bitcoin/rust-bitcoin/pull/3666)
* Fix and improve `locktime` modules
  * Modify locktime `serde` implementations [#4511](https://github.com/rust-bitcoin/rust-bitcoin/pull/4511)
  * Improve lock times - fix off-by-one bug [#4468](https://github.com/rust-bitcoin/rust-bitcoin/pull/4468)
  * Do lock time renames [#4462](https://github.com/rust-bitcoin/rust-bitcoin/pull/4462)
* Make block-related types have private inner fields [#4508](https://github.com/rust-bitcoin/rust-bitcoin/pull/4508)
* `Timestamp`/`BlockTime`
  * Add `Timestamp` newtype [#4092](https://github.com/rust-bitcoin/rust-bitcoin/pull/4092)
  * Rename then new `Timestamp` type to `BlockTime` [#4219](https://github.com/rust-bitcoin/rust-bitcoin/pull/4219)
* Add µBTC as a recognized str form of a `MicroBitcoin` `Denomination` [#3943](https://github.com/rust-bitcoin/rust-bitcoin/pull/3943)
* Remove `InputString` from the public API [#3905](https://github.com/rust-bitcoin/rust-bitcoin/pull/3905)
* Hide the remaining public macros [#3867](https://github.com/rust-bitcoin/rust-bitcoin/pull/3867)
* Change method return type for `to_unsigned()` [#3769](https://github.com/rust-bitcoin/rust-bitcoin/pull/3769)
* Change paramater type used for whole bitcoin [#3744](https://github.com/rust-bitcoin/rust-bitcoin/pull/3744)
* Add `Weight::to_kwu_ceil` [#3740](https://github.com/rust-bitcoin/rust-bitcoin/pull/3740)
* Replace `String` with `InputString` [#3559](https://github.com/rust-bitcoin/rust-bitcoin/pull/3559)

## Changes relate to error types

* Close the hex parse errors [#3673](https://github.com/rust-bitcoin/rust-bitcoin/pull/3673)

## Improved support for `Arbitrary`

* Implement `Arbitrary` for `units` types [#3777](https://github.com/rust-bitcoin/rust-bitcoin/pull/3777)
* Add `Arbitrary` to `Weight` [#3257](https://github.com/rust-bitcoin/rust-bitcoin/pull/3257)

# 0.2.0 - 2024-09-18

* Bump MSRV to 1.63.0 [#3100](https://github.com/rust-bitcoin/rust-bitcoin/pull/3100)
* Remove re-export of `ParseIntError` [#3069](https://github.com/rust-bitcoin/rust-bitcoin/pull/3069)
* Improve docs [#2957](https://github.com/rust-bitcoin/rust-bitcoin/pull/2957)
* Fix `Amount` decimals handling [#2951](https://github.com/rust-bitcoin/rust-bitcoin/pull/2951)
* Remove `Denomination::MilliSatoshi` [#2870](https://github.com/rust-bitcoin/rust-bitcoin/pull/2870)
* Document that the implementation of `Display` for `Amount` is unstable [#3323](https://github.com/rust-bitcoin/rust-bitcoin/pull/3323)
* Add a condition for parsing zero from string when not denominated [#3346](https://github.com/rust-bitcoin/rust-bitcoin/pull/3346)
* Enforce displaying `Amount` with trailing zeros [#2604](https://github.com/rust-bitcoin/rust-bitcoin/pull/2604)
* Fix `Amount` decimals handling [#2951](https://github.com/rust-bitcoin/rust-bitcoin/pull/2951)
* Error instead of panic when `Time::from_second_ceil` input is too large [#3052](https://github.com/rust-bitcoin/rust-bitcoin/pull/3052)
* Remove re-export of `ParseIntError` [#3069](https://github.com/rust-bitcoin/rust-bitcoin/pull/3069)
* Add `FeeRate` addition and subtraction traits [#3381](https://github.com/rust-bitcoin/rust-bitcoin/pull/3381)
* Add `BlockHeight` and `BlockInterval` types [#2615](https://github.com/rust-bitcoin/rust-bitcoin/pull/2615)

## Additional test infrastructure:`Arbitrary`

This release we started adding implementations of
[`arbitrary::Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html).

Types implemented: `Amount`, `SignedAmount`, `FeeRate`, and `Weight`.

In the following PRs:

* [#3305](https://github.com/rust-bitcoin/rust-bitcoin/pull/3015)
* [#3257](https://github.com/rust-bitcoin/rust-bitcoin/pull/3257)
* [#3247](https://github.com/rust-bitcoin/rust-bitcoin/pull/3274)

## 0.1.2 - 2024-07-01

* Remove enable of `alloc` feature in the `internals` dependency.

Note, the bug fixed by this release was introduced in
[#2655](https://github.com/rust-bitcoin/rust-bitcoin/pull/2655) and
was incorrect because we have an `alloc` feature that enables
`internals/alloc`.

`v0.1.1` will be yanked for this reason.

## 0.1.1 - 2024-04-04

* Enable "alloc" feature for `internals` dependency - enables caching
  of parsed input strings in a couple of `amount` error types.

## 0.1.0 - Initial Release - 2024-04-03

Initial release of the `bitcoin-units` crate. These unit types are
integer wrapper types used by the `rust-bitcoin` ecosystem. Note
please that this release relies heavily on the "alloc" feature.

The main types are:

- `Amount`
- `locktime::absolute::{Height, Time}`
- `locktime::relative::{Height, Time}`
- `FeeRate`
- `Weight`

## 0.0.0 - Placeholder release

Empty crate to reserve the name on crates.io