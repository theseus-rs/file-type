use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_251: FileType = FileType {
    file_format: &FileFormat {
        id: 251,
        source_type: SourceType::Linguist,
        name: "Nit",
        extensions: &["nit"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
