use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3877603713: FileType = FileType {
    file_format: &FileFormat {
        id: 3_877_603_713,
        source_type: SourceType::Iana,
        name: "prs.implied-object+yaml",
        extensions: &[],
        media_types: &["application/prs.implied-object+yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
