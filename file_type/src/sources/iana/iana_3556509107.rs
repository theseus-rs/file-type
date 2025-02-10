use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3556509107: FileType = FileType {
    file_format: &FileFormat {
        id: 3_556_509_107,
        source_type: SourceType::Iana,
        name: "clearmode",
        extensions: &[],
        media_types: &["audio/clearmode"],
        signatures: &[],
        related_formats: &[],
    },
};
