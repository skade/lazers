```rust
#[derive(Deserialize, Serialize)]
pub struct HistoryEntry {
    doc_write_failures: i64,
    docs_read: i64,
    docs_written: i64,
    end_last_seq: i64,
    end_time: String,// should be date
    missing_checked: i64,
    missing_found: i64,
    recorded_seq: i64,
    session_id: String,
    start_last_seq: i64,
    start_time: String,// should be Date
}

#[derive(Deserialize, Serialize)]
pub struct ReplicationLog {
    history: Vec<HistoryEntry>,
    replication_id_version: i8,
    session_id: String,
    source_last_seq: i64
}

#[cfg(test)]
mod tests {
    use serde_json as json;
    use super::HistoryEntry;
    use super::ReplicationLog;

    #[test]
    fn parses_history_info() {
        json::from_str::<HistoryEntry>(r#"{"doc_write_failures": 0, "docs_read": 2, "docs_written": 2, "end_last_seq": 5, "end_time": "Thu, 10 Oct 2013 05:56:38 GMT", "missing_checked": 2, "missing_found": 2, "recorded_seq": 5, "session_id": "d5a34cbbdafa70e0db5cb57d02a6b955", "start_last_seq": 3, "start_time": "Thu, 10 Oct 2013 05:56:38 GMT"}"#)
            .unwrap();
    }
    
    #[test]
    fn parses_replication_log() {
        json::from_str::<ReplicationLog>(r#"{"_id": "_local/b3e44b920ee2951cb2e123b63044427a", "_rev": "0-8", "history": [{"doc_write_failures": 0, "docs_read": 2, "docs_written": 2, "end_last_seq": 5, "end_time": "Thu,  10 Oct 2013 05:56:38 GMT", "missing_checked": 2, "missing_found": 2, "recorded_seq": 5, "session_id": "d5a34cbbdafa70e0db5cb57d02a6b955", "start_last_seq": 3, "start_time": "Thu,  10 Oct 2013 05:56:38 GMT"}, {"doc_write_failures": 0, "docs_read": 1, "docs_written": 1, "end_last_seq": 3, "end_time": "Thu,  10 Oct 2013 05:56:12 GMT", "missing_checked": 1, "missing_found": 1, "recorded_seq": 3, "session_id": "11a79cdae1719c362e9857cd1ddff09d", "start_last_seq": 2, "start_time": "Thu,  10 Oct 2013 05:56:12 GMT"}, {"doc_write_failures": 0, "docs_read": 2, "docs_written": 2, "end_last_seq": 2, "end_time": "Thu,  10 Oct 2013 05:56:04 GMT", "missing_checked": 2, "missing_found": 2, "recorded_seq": 2, "session_id": "77cdf93cde05f15fcb710f320c37c155", "start_last_seq": 0, "start_time": "Thu,  10 Oct 2013 05:56:04 GMT"}], "replication_id_version": 3, "session_id": "d5a34cbbdafa70e0db5cb57d02a6b955", "source_last_seq": 5}"#)
            .unwrap();
    }
}
```