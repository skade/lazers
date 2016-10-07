# The CouchDB database document

```
{"db_name":"test","doc_count":0,"doc_del_count":0,"update_seq":0,"purge_seq":0,"compact_running":false,"disk_size":79,"data_size":0,"instance_start_time":"1475608668265086","disk_format_version":6,"committed_update_seq":0}
```

```rust
use serde;
use serde::de::Deserialize;

#[derive(Debug)]
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

impl Deserialize for CouchDBInfo {
    fn deserialize<D>(deserializer: &mut D) -> Result<CouchDBInfo, D::Error>
        where D: serde::Deserializer
    {
        deserializer.deserialize(CouchDBInfoVisitor)
    }
}

impl serde::Deserialize for CouchDBInfoField {
    fn deserialize<D>(deserializer: &mut D) -> Result<CouchDBInfoField, D::Error>
        where D: serde::de::Deserializer
    {
        struct CouchDBInfoFieldVisitor;

        impl serde::de::Visitor for CouchDBInfoFieldVisitor {
            type Value = CouchDBInfoField;

            fn visit_str<E>(&mut self, value: &str) -> Result<CouchDBInfoField, E>
                where E: serde::de::Error
            {
                match value {
                    "db_name" => Ok(CouchDBInfoField::DbName),
                    "doc_count" => Ok(CouchDBInfoField::DocCount),
                    "doc_del_count" => Ok(CouchDBInfoField::DocDelCount),
                    "update_seq" => Ok(CouchDBInfoField::UpdateSeq),
                    "purge_seq" => Ok(CouchDBInfoField::PurgeSeq),
                    "compact_running" => Ok(CouchDBInfoField::CompactRunning),
                    "disk_size" => Ok(CouchDBInfoField::DiskSize),
                    "data_size" => Ok(CouchDBInfoField::DataSize),
                    "instance_start_time" => Ok(CouchDBInfoField::InstanceStartTime),
                    "disk_format_version" => Ok(CouchDBInfoField::DiskFormatVersion),
                    "committed_update_seq" => Ok(CouchDBInfoField::CommittedUpdateSeq),
                    _ => {
                        Err(serde::de::Error::unknown_field(format!("expected a document info field
                                                                     field, got: {}",
                                                                    value)
                            .as_ref()))
                    }
                }
            }
        }

        deserializer.deserialize(CouchDBInfoFieldVisitor)
    }
}

struct CouchDBInfoVisitor;

impl serde::de::Visitor for CouchDBInfoVisitor {
    type Value = CouchDBInfo;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<CouchDBInfo, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut db_name = None;
        let mut doc_count = None;
        let mut doc_del_count = None;
        let mut update_seq = None;
        let mut purge_seq = None;
        let mut compact_running = None;
        let mut disk_size = None;
        let mut data_size = None;
        let mut instance_start_time = None;
        let mut disk_format_version = None;
        let mut committed_update_seq = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(CouchDBInfoField::DbName) => {
                    db_name = Some(try!(visitor.visit_value()));
                },
                Some(CouchDBInfoField::DocCount) => {
                    doc_count = Some(try!(visitor.visit_value()));
                },
                Some(CouchDBInfoField::DocDelCount) => {
                    doc_del_count = Some(try!(visitor.visit_value()));
                },
                Some(CouchDBInfoField::UpdateSeq) => {
                    update_seq = Some(try!(visitor.visit_value()));
                },
                Some(CouchDBInfoField::PurgeSeq) => {
                    purge_seq = Some(try!(visitor.visit_value()));
                },
                Some(CouchDBInfoField::CompactRunning) => {
                    compact_running = Some(try!(visitor.visit_value()));
                },
                Some(CouchDBInfoField::DiskSize) => {
                    disk_size = Some(try!(visitor.visit_value()));
                },
                Some(CouchDBInfoField::DataSize) => {
                    data_size = Some(try!(visitor.visit_value()));
                },
                Some(CouchDBInfoField::InstanceStartTime) => {
                    instance_start_time = Some(try!(visitor.visit_value()));
                },
                Some(CouchDBInfoField::DiskFormatVersion) => {
                    disk_format_version = Some(try!(visitor.visit_value()));
                },
                Some(CouchDBInfoField::CommittedUpdateSeq) => {
                    committed_update_seq = Some(try!(visitor.visit_value()));
                },
                None => break
            }
        }

        let db_name = match db_name {
            Some(db_name) => db_name,
            None => try!(visitor.missing_field("db_name")),
        };

        let doc_count = match doc_count {
            Some(doc_count) => doc_count,
            None => try!(visitor.missing_field("doc_count")),
        };

        let doc_del_count = match doc_del_count {
            Some(doc_del_count) => doc_del_count,
            None => try!(visitor.missing_field("doc_del_count")),
        };

        let update_seq = match update_seq {
            Some(update_seq) => update_seq,
            None => try!(visitor.missing_field("update_seq")),
        };

        let purge_seq = match purge_seq {
            Some(purge_seq) => purge_seq,
            None => try!(visitor.missing_field("purge_seq")),
        };

        let compact_running = match compact_running {
            Some(compact_running) => compact_running,
            None => try!(visitor.missing_field("compact_running")),
        };

        let disk_size = match disk_size {
            Some(disk_size) => disk_size,
            None => try!(visitor.missing_field("disk_size")),
        };

        let data_size = match data_size {
            Some(data_size) => data_size,
            None => try!(visitor.missing_field("data_size")),
        };

        let instance_start_time = match instance_start_time {
            Some(instance_start_time) => instance_start_time,
            None => try!(visitor.missing_field("instance_start_time")),
        };

        let disk_format_version = match disk_format_version {
            Some(disk_format_version) => disk_format_version,
            None => try!(visitor.missing_field("disk_format_version")),
        };

        let committed_update_seq = match committed_update_seq {
            Some(committed_update_seq) => committed_update_seq,
            None => try!(visitor.missing_field("committed_update_seq")),
        };

        try!(visitor.end());

        Ok(CouchDBInfo {
            db_name: db_name,
            doc_count: doc_count,
            doc_del_count: doc_del_count,
            update_seq: update_seq,
            purge_seq: purge_seq,
            compact_running: compact_running,
            disk_size: disk_size,
            data_size: data_size,
            instance_start_time: instance_start_time,
            disk_format_version: disk_format_version,
            committed_update_seq: committed_update_seq
        })
    }
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
