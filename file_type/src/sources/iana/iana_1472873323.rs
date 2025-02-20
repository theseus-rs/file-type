use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1472873323: FileType = FileType {
    file_format: &FileFormat {
        id: 1_472_873_323,
        source_type: SourceType::Iana,
        name: "swid+xml",
        extensions: &[],
        media_types: &["application/swid+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
