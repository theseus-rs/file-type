use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_155: FileType = FileType {
    file_format: &FileFormat {
        id: 155,
        source_type: SourceType::Linguist,
        name: "Handlebars",
        extensions: &["handlebars", "hbs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
