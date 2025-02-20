use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1159758451: FileType = FileType {
    file_format: &FileFormat {
        id: 1_159_758_451,
        source_type: SourceType::Iana,
        name: "3gpp-tt",
        extensions: &[],
        media_types: &["video/3gpp-tt"],
        signatures: &[],
        related_formats: &[],
    },
};
