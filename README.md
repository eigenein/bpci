# `bpci`

Binomial proportion confidence intervals.

[![Crates.io](https://img.shields.io/crates/v/bpci)](https://crates.io/crates/bpci)
[![Last commit](https://img.shields.io/github/last-commit/eigenein/bpci)](https://github.com/eigenein/bpci/commits/master)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/eigenein/bpci/Check)](https://github.com/eigenein/bpci/actions)
![License: MIT](https://img.shields.io/crates/l/bpci)

## Samples

### By size and number of successes

```rust
use bpci::NSuccessesSample;

fn main() {
    // 10 successes out of 20 trials:
    let sample = NSuccessesSample::new(20, 10).unwrap();
}
```

### By size and proportion

```rust
use bpci::ProportionSample;

fn main() {
    // 20 trials with 0.5 success rate:
    let sample = ProportionSample::new(20, 0.5).unwrap();
}
```

## Intervals

### [Wilson score interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Wilson_score_interval)

```rust
use bpci::*;
use approx::assert_relative_eq;

fn main() {
    let sample = ProportionSample::new(100, 0.25).unwrap();
    let interval = sample.wilson_score(1.960); // 95%
    assert_relative_eq!(interval.lower(), 0.1754509400372426);
    assert_relative_eq!(interval.upper(), 0.3430464637007583);
}
```

### [Wilson score interval with continuity correction](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Wilson_score_interval_with_continuity_correction)

```rust
use bpci::*;
use approx::assert_relative_eq;

fn main() {
    let sample = ProportionSample::new(100, 0.25).unwrap();
    let interval = sample.wilson_score_with_cc(1.960); // 95%
    assert_relative_eq!(interval.lower(), 0.17117438961361867);
    assert_relative_eq!(interval.upper(), 0.34838596518606424);
}
```

### [Agresti–Coull interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Agresti%E2%80%93Coull_interval)

```rust
use bpci::*;
use approx::assert_relative_eq;

fn main() {
    let sample = ProportionSample::new(40, 0.25).unwrap();
    let interval = sample.agresti_coull(1.960); // 95%
    assert_relative_eq!(interval.mean, 0.2719061348125981);
    assert_relative_eq!(interval.margin, 0.1317091851034039);
}
```

```rust
use bpci::*;
use approx::assert_relative_eq;

fn main() {
    let sample = NSuccessesSample::new(40, 10).unwrap();
    let interval = sample.agresti_coull(1.960); // 95%
    assert_relative_eq!(interval.mean, 0.2719061348125981);
    assert_relative_eq!(interval.margin, 0.1317091851034039);
}
```

### [Normal approximation interval or Wald interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Normal_approximation_interval_or_Wald_interval)

```rust
use bpci::*;
use approx::assert_relative_eq;

fn main() {
    let sample = ProportionSample::new(100, 0.3).unwrap();
    let interval = sample.wald(1.960); // 95%
    assert_relative_eq!(interval.lower(), 0.2101815163788655);
    assert_relative_eq!(interval.upper(), 0.38981848362113447);
}
```
