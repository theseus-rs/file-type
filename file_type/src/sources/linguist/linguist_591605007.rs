use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_591605007: FileType = FileType {
    file_format: &FileFormat {
        id: 591_605_007,
        source_type: SourceType::Linguist,
        name: "Asymptote",
        extensions: &["asy"],
        media_types: &["text/x-kotlin"],
        signatures: &[],
        related_formats: &[],
    },
};
