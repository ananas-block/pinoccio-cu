# Pinoccio CU vs Solana Program CU

copy spl_noop.so to target/deploy/

Switch between pinoccio and solana program by changing the default feature in program Cargo.toml.

`cargo test-sbf`

## Bench Cpi Pinoccio

#### bench cpis 8 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked true         | 1,118    |
| Invoke checked false        | 1,061    |
| Invoke signed checked true  | 1,134    |
| Invoke signed checked false | 1,076    |
| Total                       | 8,165    |

#### bench cpis 3240 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked true         | 1,130    |
| Invoke checked false        | 1,073    |
| Invoke signed checked true  | 1,146    |
| Invoke signed checked false | 1,088    |
| Total                       | 8,213    |

## Bench Cpi Solana Program

#### bench cpis  8 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked        true  | 1,416    |
| Invoke unchecked      false | 1,321    |
| Invoke signed checked true  | 1,418    |
| Invoke signed uncheck false | 1,333    |
| Total                       | 9,825    |

#### bench cpis 3240 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked        true  | 1,430    |
| Invoke unchecked      false | 1,335    |
| Invoke signed checked true  | 1,432    |
| Invoke signed uncheck false | 1,347    |
| Total                       | 9,881    |
