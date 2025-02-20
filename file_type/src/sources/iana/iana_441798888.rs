use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_441798888: FileType = FileType {
    file_format: &FileFormat {
        id: 441_798_888,
        source_type: SourceType::Iana,
        name: "vnd.apache.thrift.json",
        extensions: &[],
        media_types: &["application/vnd.apache.thrift.json"],
        signatures: &[],
        related_formats: &[],
    },
};
