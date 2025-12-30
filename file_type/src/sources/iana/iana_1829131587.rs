use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1829131587: FileType = FileType {
    file_format: &FileFormat {
        id: 1_829_131_587,
        source_type: SourceType::Iana,
        name: "protobuf+json",
        extensions: &[],
        media_types: &["application/protobuf+json"],
        signatures: &[],
        related_formats: &[],
    },
};
