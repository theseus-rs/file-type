use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_728656917: FileType = FileType {
    file_format: &FileFormat {
        id: 728_656_917,
        source_type: SourceType::Iana,
        name: "G723",
        extensions: &[],
        media_types: &["audio/G723"],
        signatures: &[],
        related_formats: &[],
    },
};
