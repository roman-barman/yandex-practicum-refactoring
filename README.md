# Yandex Practicum Refactoring

A Rust library and CLI tool for parsing and filtering structured log files from a fictional asset exchange platform. Built as a refactoring exercise for Yandex Practicum.

## Overview

The project demonstrates iterative refactoring of a log processing pipeline. It provides:

- A custom **parser combinator library** (no external dependencies)
- Typed **log line parsing** for System and App log categories
- **Filtering** by request ID, error type, or exchange operations (buy/sell)
- A **CLI binary** that reads log files from disk

## Log Format

Each log line follows one of these patterns:

```
System::Trace <kind> <data> requestid=<N>
System::Error <kind> "<message>" requestid=<N>
App::Trace <kind> <data> requestid=<N>
App::Error <kind> "<message>" requestid=<N>
App::Journal <operation> <data> requestid=<N>
```

### Log kinds

| Category       | Kind          | Description                          |
|----------------|---------------|--------------------------------------|
| System::Trace  | SendRequest   | Outgoing HTTP request to exchange    |
| System::Trace  | GetResponse   | Incoming HTTP response               |
| System::Error  | NetworkError  | Network-level failure                |
| System::Error  | AccessDenied  | Authorization failure                |
| App::Trace     | Connect       | Exchange connection established      |
| App::Trace     | SendRequest   | Application-level request            |
| App::Trace     | GetResponse   | Application-level response           |
| App::Trace     | Check         | Local validation of announcements    |
| App::Error     | SystemError   | Internal system error                |
| App::Error     | LackOf        | Missing asset or seller              |
| App::Journal   | CreateUser    | User created with authorized capital |
| App::Journal   | DeleteUser    | User deleted                         |
| App::Journal   | RegisterAsset | Asset registered with liquidity      |
| App::Journal   | DeleteAsset   | Asset deleted                        |
| App::Journal   | Deposit       | USD deposited for user               |
| App::Journal   | Withdraw      | USD withdrawn for user               |
| App::Journal   | BuyAsset      | Asset purchase recorded              |
| App::Journal   | SellAsset     | Asset sale recorded                  |

## Data Model

- **User** — `user_id`, name
- **Asset** — `asset_id`, name; quantity tracked per user
- **Bucket** — `asset_id` + `count` (a position in a trade)
- **UserBucket** / **UserBuckets** — user with one or more buckets
- **Announcements** — list of `UserBuckets` (the order book)
- **UserCash** — user's USD balance

## Library API

```rust
use analysis::{read_log, ReadMode};

// Read and parse all log lines from any `Read` source
let logs = read_log(file, ReadMode::All, vec![]);

// Filter to errors only
let errors = read_log(file, ReadMode::Errors, vec![]);

// Filter to buy/sell journal entries
let trades = read_log(file, ReadMode::Exchanges, vec![]);

// Filter by specific request IDs
let subset = read_log(file, ReadMode::All, vec![3, 4]);
```

Parse individual entities directly:

```rust
use analysis::entities::{just_parse, Announcements};

let input = r#"[UserBackets{"user_id":"Bob","backets":[Backet{"asset_id":"milk","count":3,},],},]"#;
let (_, announcements) = just_parse::<Announcements>(input).unwrap();
```

## CLI Usage

```
cargo run -- <path-to-log-file>
```

Prints all parsed log lines to stdout. If no file argument is provided, runs a built-in parsing demo.

## Project Structure

```
src/
├── lib.rs              # Public API: read_log, ReadMode, LogIterator
├── main.rs             # CLI entry point
├── parsable.rs         # Parsable trait
├── parse.rs            # Parser trait + re-exports of all combinators
├── parse/              # Parser combinator implementations
│   ├── all_parse.rs
│   ├── alt_parse.rs
│   ├── delimited_parse.rs
│   ├── key_value_parse.rs
│   ├── list_parse.rs
│   ├── map_parse.rs
│   ├── permutation_parse.rs
│   ├── preceded_parse.rs
│   ├── quoted_tag_parse.rs
│   ├── std_parse.rs    # Parsers for u8, i32, u32
│   ├── strip_whitespace_parse.rs
│   ├── tag_parse.rs
│   ├── take_parse.rs
│   └── unquote_parse.rs
├── logs.rs             # LogLineParser, re-exports
├── logs/               # Log type definitions
│   ├── log_line.rs
│   ├── log_kind.rs
│   ├── app_log_kind.rs
│   ├── app_log_trace_kind.rs
│   ├── app_log_journal_kind.rs
│   ├── app_log_error_kind.rs
│   ├── system_log_kind.rs
│   ├── system_log_trace_kind.rs
│   └── system_log_error_kind.rs
├── entities.rs         # Public entity types, just_parse helper
└── entities/           # Entity definitions
    ├── announcements.rs
    ├── asset_dsc.rs
    ├── auth_data.rs
    ├── bucket.rs
    ├── status.rs
    ├── user_bucket.rs
    ├── user_buckets.rs
    └── user_cash.rs
```

## Building and Testing

```bash
cargo build
cargo test
cargo run -- path/to/logfile.log
```

No external dependencies — only the Rust standard library.
