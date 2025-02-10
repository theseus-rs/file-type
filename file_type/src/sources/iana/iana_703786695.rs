use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_703786695: FileType = FileType {
    file_format: &FileFormat {
        id: 703_786_695,
        source_type: SourceType::Iana,
        name: "MELP600",
        extensions: &[],
        media_types: &["audio/MELP600"],
        signatures: &[],
        related_formats: &[],
    },
};
