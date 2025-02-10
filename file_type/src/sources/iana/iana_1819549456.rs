use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1819549456: FileType = FileType {
    file_format: &FileFormat {
        id: 1_819_549_456,
        source_type: SourceType::Iana,
        name: "SMV-QCP",
        extensions: &[],
        media_types: &["audio/SMV-QCP"],
        signatures: &[],
        related_formats: &[],
    },
};
