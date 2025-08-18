use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_95: FileType = FileType {
    file_format: &FileFormat {
        id: 95,
        source_type: SourceType::Linguist,
        name: "EJS",
        extensions: &["ect", "ejs", "ejs.t", "jst"],
        media_types: &["application/x-ejs"],
        signatures: &[],
        related_formats: &[],
    },
};
