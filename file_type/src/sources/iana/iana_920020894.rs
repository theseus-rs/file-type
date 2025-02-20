use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_920020894: FileType = FileType {
    file_format: &FileFormat {
        id: 920_020_894,
        source_type: SourceType::Iana,
        name: "vnd.nokia.landmark+wbxml",
        extensions: &[],
        media_types: &["application/vnd.nokia.landmark+wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
