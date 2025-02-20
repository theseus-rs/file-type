use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4121617177: FileType = FileType {
    file_format: &FileFormat {
        id: 4_121_617_177,
        source_type: SourceType::Iana,
        name: "im-iscomposing+xml",
        extensions: &[],
        media_types: &["application/im-iscomposing+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
