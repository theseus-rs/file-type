use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_389477596: FileType = FileType {
    file_format: &FileFormat {
        id: 389_477_596,
        source_type: SourceType::Linguist,
        name: "AngelScript",
        extensions: &["angelscript", "as"],
        media_types: &["text/x-c++src"],
        signatures: &[],
        related_formats: &[],
    },
};
