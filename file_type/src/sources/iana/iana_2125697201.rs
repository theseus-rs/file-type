use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2125697201: FileType = FileType {
    file_format: &FileFormat {
        id: 2_125_697_201,
        source_type: SourceType::Iana,
        name: "vnd.3M.Post-it-Notes",
        extensions: &[],
        media_types: &["application/vnd.3M.Post-it-Notes"],
        signatures: &[],
        related_formats: &[],
    },
};
