use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1308395743: FileType = FileType {
    file_format: &FileFormat {
        id: 1_308_395_743,
        source_type: SourceType::Iana,
        name: "basic",
        extensions: &[],
        media_types: &["audio/basic"],
        signatures: &[],
        related_formats: &[],
    },
};
