use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2403236095: FileType = FileType {
    file_format: &FileFormat {
        id: 2_403_236_095,
        source_type: SourceType::Iana,
        name: "org",
        extensions: &[],
        media_types: &["text/org"],
        signatures: &[],
        related_formats: &[],
    },
};
