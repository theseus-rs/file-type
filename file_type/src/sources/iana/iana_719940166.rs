use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_719940166: FileType = FileType {
    file_format: &FileFormat {
        id: 719_940_166,
        source_type: SourceType::Iana,
        name: "vnd.is-xpr",
        extensions: &[],
        media_types: &["application/vnd.is-xpr"],
        signatures: &[],
        related_formats: &[],
    },
};
