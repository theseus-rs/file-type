use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1283070147: FileType = FileType {
    file_format: &FileFormat {
        id: 1_283_070_147,
        source_type: SourceType::Iana,
        name: "vnd.xmpie.ppkg",
        extensions: &[],
        media_types: &["application/vnd.xmpie.ppkg"],
        signatures: &[],
        related_formats: &[],
    },
};
