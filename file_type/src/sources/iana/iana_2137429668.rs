use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2137429668: FileType = FileType {
    file_format: &FileFormat {
        id: 2_137_429_668,
        source_type: SourceType::Iana,
        name: "vnd.wmf.bootstrap",
        extensions: &[],
        media_types: &["application/vnd.wmf.bootstrap"],
        signatures: &[],
        related_formats: &[],
    },
};
