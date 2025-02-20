use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2837789170: FileType = FileType {
    file_format: &FileFormat {
        id: 2_837_789_170,
        source_type: SourceType::Iana,
        name: "vnd.genozip",
        extensions: &[],
        media_types: &["application/vnd.genozip"],
        signatures: &[],
        related_formats: &[],
    },
};
