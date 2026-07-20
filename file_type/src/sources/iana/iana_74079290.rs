use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_74079290: FileType = FileType {
    file_format: &FileFormat {
        id: 74_079_290,
        source_type: SourceType::Iana,
        name: "vnd.rego",
        extensions: &[],
        media_types: &["application/vnd.rego"],
        signatures: &[],
        related_formats: &[],
    },
};
