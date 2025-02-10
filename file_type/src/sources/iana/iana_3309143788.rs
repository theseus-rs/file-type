use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3309143788: FileType = FileType {
    file_format: &FileFormat {
        id: 3_309_143_788,
        source_type: SourceType::Iana,
        name: "vnd.apache.thrift.binary",
        extensions: &[],
        media_types: &["application/vnd.apache.thrift.binary"],
        signatures: &[],
        related_formats: &[],
    },
};
