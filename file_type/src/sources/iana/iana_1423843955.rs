use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1423843955: FileType = FileType {
    file_format: &FileFormat {
        id: 1_423_843_955,
        source_type: SourceType::Iana,
        name: "alto-tipsparams+json",
        extensions: &[],
        media_types: &["application/alto-tipsparams+json"],
        signatures: &[],
        related_formats: &[],
    },
};
