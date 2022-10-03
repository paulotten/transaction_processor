## Usage

To run the program type:

```
cargo run -- transactions.csv > accounts.csv
```

To run the tests type:

```
cargo test
```

## Assumptions / Design Choices

When reading the input file, line numbers are tracked for error reporting. Line numbers are stored as `u32`, the same as transaction ids. Hopefully you don't toss me more lines of input than you have valid transaction ids for. (You could however with disputes/resolves/chargebacks which by design reuse transaction ids. You could also incorrectly reuse transaction ids for deposits/withdrawals.)

When reading the input file, errors processing individual lines are logged to stderr. The program then continues to the remaining lines.

Reusing transaction ids for deposits/withdrawals is assumed to be invalid.

Amounts are stored as `i64` "cents". "Cents" in for the purpose of this program represent 1/10,000th of an amount. Amounts typically should not be negative. A deposit, followed by a withdrawal, followed by a dispute plus chargeback could however result in a negative account balance.

Disputes (and resolutions and chargebacks) only make sense for deposits. Disputing a withdrawal would increase the amount of available funds which is the opposite of what a dispute is supposed to do.

Once an account is locked you cannot do anything (deposit/withdrawal/dispute/resolve/chargeback) to it. It assumed that manual intervention is required to unlock an account.

Disputes/resolutions/chargebacks with amounts are assumed to be invalid and are rejected.

Withdrawal data is never read from the list of transactions (TransactionMap). This program would work without storing it at all. This is information is stored because it is assumed to be useful outside the scope of the program. (Disputes/resolutions/chargebacks would be stored to if they had their own unique transaction ids.)
