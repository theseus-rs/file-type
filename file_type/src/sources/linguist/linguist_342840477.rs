use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_342840477: FileType = FileType {
    file_format: &FileFormat {
        id: 342_840_477,
        source_type: SourceType::Linguist,
        name: "Easybuild",
        extensions: &["eb"],
        media_types: &["text/x-python"],
        signatures: &[],
        related_formats: &[],
    },
};
