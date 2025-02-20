use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_423816041: FileType = FileType {
    file_format: &FileFormat {
        id: 423_816_041,
        source_type: SourceType::Iana,
        name: "wspolicy+xml",
        extensions: &[],
        media_types: &["application/wspolicy+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
