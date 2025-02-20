use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1156879722: FileType = FileType {
    file_format: &FileFormat {
        id: 1_156_879_722,
        source_type: SourceType::Iana,
        name: "prs.implied-executable",
        extensions: &[],
        media_types: &["application/prs.implied-executable"],
        signatures: &[],
        related_formats: &[],
    },
};
