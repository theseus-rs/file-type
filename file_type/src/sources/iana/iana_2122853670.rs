use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2122853670: FileType = FileType {
    file_format: &FileFormat {
        id: 2_122_853_670,
        source_type: SourceType::Iana,
        name: "vnd.vri",
        extensions: &[],
        media_types: &["text/vnd.vri"],
        signatures: &[],
        related_formats: &[],
    },
};
