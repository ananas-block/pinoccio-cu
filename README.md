# Pinoccio CU vs Solana Program CU

copy spl_noop.so to target/deploy/

Switch between pinoccio and solana program by changing the default feature in program Cargo.toml.

`cargo test-sbf`

## Bench Cpi Pinoccio

#### bench cpis 8 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked true         | 1115     |
| Invoke checked false        | 1571     |
| Invoke signed checked true  | 1677     |
| Invoke signed checked false | 1629     |

#### bench cpis 1024 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked true         | 1119     |
| Invoke checked false        | 1575     |
| Invoke signed checked true  | 1681     |
| Invoke signed checked false | 1633     |


## Bench Cpi Solana Program

#### bench cpis  8 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked        true  | 1416     |
| Invoke unchecked      false | 1321     |
| Invoke signed checked true  | 1420     |
| Invoke signed uncheck false | 1332     |

#### bench cpis 1024 bytes

| name                        | CU usage |
|-----------------------------|----------|
| Invoke checked        true  | 1155     |
| Invoke unchecked      false | 1323     |
| Invoke signed checked true  | 1422     |
| Invoke signed uncheck false | 1334     |