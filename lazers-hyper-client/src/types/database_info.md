# The CouchDB database document

```
{"db_name":"test","doc_count":0,"doc_del_count":0,"update_seq":0,"purge_seq":0,"compact_running":false,"disk_size":79,"data_size":0,"instance_start_time":"1475608668265086","disk_format_version":6,"committed_update_seq":0}
```

```rust
#[derive(Debug, Deserialize)]
pub struct CouchDBInfo {
    pub db_name: String,
    pub doc_count: u64,
    pub doc_del_count: u64,
    pub update_seq: u64,
    pub purge_seq: u64,
    pub compact_running: bool,
    pub disk_size: u64,
    pub data_size: u64,
    pub instance_start_time: String,
    pub disk_format_version: u64,
    pub committed_update_seq: u64
}

enum CouchDBInfoField {
    DbName,
    DocCount,
    DocDelCount,
    UpdateSeq,
    PurgeSeq,
    CompactRunning,
    DiskSize,
    DataSize,
    InstanceStartTime,
    DiskFormatVersion,
    CommittedUpdateSeq,
}

#[cfg(test)]
mod tests {
    use serde_json as json;
    use super::CouchDBInfo;

    #[test]
    fn parses_couchdb_info() {
        json::from_str::<CouchDBInfo>("{\"db_name\":\"test\",\"doc_count\":0,\"doc_del_count\":0,\"update_seq\":0,\"purge_seq\":0,\"compact_running\":false,\"disk_size\":79,\"data_size\":0,\"instance_start_time\":\"1475608668265086\",\"disk_format_version\":6,\"committed_update_seq\":0}")
            .unwrap();
    }
}
```
