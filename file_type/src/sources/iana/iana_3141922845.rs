use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3141922845: FileType = FileType {
    file_format: &FileFormat {
        id: 3_141_922_845,
        source_type: SourceType::Iana,
        name: "MP1S",
        extensions: &[],
        media_types: &["video/MP1S"],
        signatures: &[],
        related_formats: &[],
    },
};
