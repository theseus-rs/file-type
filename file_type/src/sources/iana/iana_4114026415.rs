use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4114026415: FileType = FileType {
    file_format: &FileFormat {
        id: 4_114_026_415,
        source_type: SourceType::Iana,
        name: "vnd.apache.thrift.compact",
        extensions: &[],
        media_types: &["application/vnd.apache.thrift.compact"],
        signatures: &[],
        related_formats: &[],
    },
};
