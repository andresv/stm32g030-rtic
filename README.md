# RTIC2 test with stm32g030

## Build

```rust
cd firmware
DEFMT_LOG=info cargo rrb rtictest
```

Fails with this if priority 2 is used for the task:
```
--> src/main.rs:30:1
 |
30 | #[app(device = embassy_stm32, peripherals = true, dispatchers = [TIM16, TIM17])]
 | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `Interrupt` in `embassy_stm32`
 |
 = note: this error originates in the attribute macro `app` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these items
 |
32 +     use crate::app::you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::Interrupt;
 |
32 +     use crate::app::you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::typelevel::Interrupt;
 |
32 +     use embassy_stm32::interrupt::Interrupt;
 |
32 +     use embassy_stm32::interrupt::typelevel::Interrupt;
```
