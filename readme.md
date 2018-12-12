# `efm32`

board support package for EFM32 Happy Gecko series from Silicon Labs

The EFM32HG register definitions are from from keil.com and provided here in /svd.

regenerate
--
```
rm -rf src && mkdir src && touch src/lib.rs && cargo gen
```

