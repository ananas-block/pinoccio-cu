# Pinoccio CU vs Solana Program CU

copy spl_noop.so to target/deploy/

Switch between pinoccio and solana program by changing the default feature in program Cargo.toml.

`cargo test-sbf`

## Entrypoint for 20 accounts

| library                     | CU usage |
|-----------------------------|----------|
| solana-nostd-entrypoint     |  306     |
| pinoccio                    |  427     |
| solana-program              |  3,585   |


## Pinoccio Cpi

#### noop cpis 8 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked true         | 1,118    |
| Invoke checked false        | 1,061    |
| Invoke signed checked true  | 1,134    |
| Invoke signed checked false | 1,076    |
| Total                       | 8,165    |

#### noop cpis 3240 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked true         | 1,130    |
| Invoke checked false        | 1,073    |
| Invoke signed checked true  | 1,146    |
| Invoke signed checked false | 1,088    |
| Total                       | 8,213    |

## Solana Program Cpi

#### noop cpis  8 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked        true  | 1,416    |
| Invoke unchecked      false | 1,321    |
| Invoke signed checked true  | 1,418    |
| Invoke signed uncheck false | 1,333    |
| Total                       | 9,825    |

#### noop cpis 3240 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked        true  | 1,430    |
| Invoke unchecked      false | 1,335    |
| Invoke signed checked true  | 1,432    |
| Invoke signed uncheck false | 1,347    |
| Total                       | 9,881    |
