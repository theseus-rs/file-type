use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4030836309: FileType = FileType {
    file_format: &FileFormat {
        id: 4_030_836_309,
        source_type: SourceType::Iana,
        name: "vnd.fut-misnet",
        extensions: &[],
        media_types: &["application/vnd.fut-misnet"],
        signatures: &[],
        related_formats: &[],
    },
};
