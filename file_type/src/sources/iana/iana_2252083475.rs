use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2252083475: FileType = FileType {
    file_format: &FileFormat {
        id: 2_252_083_475,
        source_type: SourceType::Iana,
        name: "vnd.musician",
        extensions: &[],
        media_types: &["application/vnd.musician"],
        signatures: &[],
        related_formats: &[],
    },
};
