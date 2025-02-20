use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2109898018: FileType = FileType {
    file_format: &FileFormat {
        id: 2_109_898_018,
        source_type: SourceType::Iana,
        name: "woff2",
        extensions: &[],
        media_types: &["font/woff2"],
        signatures: &[],
        related_formats: &[],
    },
};
