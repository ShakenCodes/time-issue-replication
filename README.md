# Time Issue Replication

## Issue

We are attempting to use timers other than TIM2 as the source of Embassy Time. TIM2 is the default selection when embassy-stm32 has the feature "stm32wl55jc-cm4" selected. Our hardware has dedicated TIM2 to another function, and we need to use either TIM16 or TIM17 as the source of Embassy Time. These timers (and, perhaps surprisingly, TIM1) are not supported as sources of Embassy Time. We have made a fork of Embassy that enables these three timers as sources of Embassy Time, at: [https://github.com/ShakenCodes/embassy]

Unfortunately, using any of TIM1, TIM16, or TIM17 operate Embassy Time at (exactly?) half speed. To demonstrate this, I've made a simple application that outputs a message every second over the RTT port.

## Desired Outcome

Find and resolve the issue that causes the TIM1, TIM16, or TIM17 to operate at half-speed.

Our suspicion is that this is an issue in the embassy-stm32 library, either in clocks setup of timer setup.

## Replication

Test case is performed by executing ```cargo embed``` at the root of this workspace.

### Correct behavior

In Cargo.toml, for dependency embassy-stm32, set the feature "time-driver-any" or "time-driver-tim2".

### Incorrect behavior

In Cargo.toml, for dependency embassy-stm32, set the feature "time-driver-tim1", "time-driver-tim16", or "time-driver-tim17".

## Hardware

This example was built for the Nucleo-STM32WL board, although it should work on any STM32 processor that includes one or all of TIM1, TIM16, and TIM17. The Cargo.toml dependency for embassy-stm32 will need to have the feature "stm32wl55jc-cm4" changed.

## end
