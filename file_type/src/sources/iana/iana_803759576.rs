use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_803759576: FileType = FileType {
    file_format: &FileFormat {
        id: 803_759_576,
        source_type: SourceType::Iana,
        name: "vnd.mozilla.xul+xml",
        extensions: &[],
        media_types: &["application/vnd.mozilla.xul+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
