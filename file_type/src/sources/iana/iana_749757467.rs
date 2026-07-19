use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_749757467: FileType = FileType {
    file_format: &FileFormat {
        id: 749_757_467,
        source_type: SourceType::Iana,
        name: "rpki-ccr",
        extensions: &[],
        media_types: &["application/rpki-ccr"],
        signatures: &[],
        related_formats: &[],
    },
};
