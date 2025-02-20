use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_338: FileType = FileType {
    file_format: &FileFormat {
        id: 338,
        source_type: SourceType::Linguist,
        name: "Sage",
        extensions: &["sage", "sagews"],
        media_types: &["text/x-python"],
        signatures: &[],
        related_formats: &[],
    },
};
