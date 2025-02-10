use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_437786028: FileType = FileType {
    file_format: &FileFormat {
        id: 437_786_028,
        source_type: SourceType::Iana,
        name: "vnd.cmles.radio-events",
        extensions: &[],
        media_types: &["audio/vnd.cmles.radio-events"],
        signatures: &[],
        related_formats: &[],
    },
};
