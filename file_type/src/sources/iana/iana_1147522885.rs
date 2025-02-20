use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1147522885: FileType = FileType {
    file_format: &FileFormat {
        id: 1_147_522_885,
        source_type: SourceType::Iana,
        name: "EDI-X12",
        extensions: &[],
        media_types: &["application/EDI-X12"],
        signatures: &[],
        related_formats: &[],
    },
};
