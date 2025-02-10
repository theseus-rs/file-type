use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_179: FileType = FileType {
    file_format: &FileFormat {
        id: 179,
        source_type: SourceType::Linguist,
        name: "Pug",
        extensions: &["jade", "pug"],
        media_types: &["text/x-pug"],
        signatures: &[],
        related_formats: &[],
    },
};
