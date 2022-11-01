# rust-cli-parquet-explorer

```sh
cargo r -- -f datasets/data.parquet
```

Output example:
```sh
➜  rust-cli-parquet-explorer git:(master) cargo r -- -f datasets/data.parquet
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/rust-cli-parquet-explorer -f datasets/data.parquet`
0 | Name: STATE | Data Type: String | Stats: ByteArray({min: Some(ByteArray { data: "ALABAMA" }), max: Some(ByteArray { data: "WYOMING" }), distinct_count: None, null_count: 0, min_max_deprecated: false}) 
1 | Name: ST_ABBR | Data Type: String | Stats: ByteArray({min: Some(ByteArray { data: "AK" }), max: Some(ByteArray { data: "WY" }), distinct_count: None, null_count: 0, min_max_deprecated: false}) 
2 | Name: COUNTY | Data Type: String | Stats: ByteArray({min: Some(ByteArray { data: "Abbeville" }), max: Some(ByteArray { data: "Ziebach" }), distinct_count: None, null_count: 0, min_max_deprecated: false}) 
```