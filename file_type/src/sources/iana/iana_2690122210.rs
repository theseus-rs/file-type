use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2690122210: FileType = FileType {
    file_format: &FileFormat {
        id: 2_690_122_210,
        source_type: SourceType::Iana,
        name: "vnd.tableschema+json",
        extensions: &[],
        media_types: &["application/vnd.tableschema+json"],
        signatures: &[],
        related_formats: &[],
    },
};
