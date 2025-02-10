use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_76276134: FileType = FileType {
    file_format: &FileFormat {
        id: 76_276_134,
        source_type: SourceType::Iana,
        name: "timestamp-reply",
        extensions: &[],
        media_types: &["application/timestamp-reply"],
        signatures: &[],
        related_formats: &[],
    },
};
