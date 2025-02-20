use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2126140606: FileType = FileType {
    file_format: &FileFormat {
        id: 2_126_140_606,
        source_type: SourceType::Iana,
        name: "vnd.nokia.n-gage.ac+xml",
        extensions: &[],
        media_types: &["application/vnd.nokia.n-gage.ac+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
