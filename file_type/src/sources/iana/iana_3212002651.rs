use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3212002651: FileType = FileType {
    file_format: &FileFormat {
        id: 3_212_002_651,
        source_type: SourceType::Iana,
        name: "vnd.yaoweme",
        extensions: &[],
        media_types: &["application/vnd.yaoweme"],
        signatures: &[],
        related_formats: &[],
    },
};
