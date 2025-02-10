use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_95: FileType = FileType {
    file_format: &FileFormat {
        id: 95,
        source_type: SourceType::Linguist,
        name: "EJS",
        extensions: &["ect", "ejs", "ejs.t", "jst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
