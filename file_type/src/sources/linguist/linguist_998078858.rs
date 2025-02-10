use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_998078858: FileType = FileType {
    file_format: &FileFormat {
        id: 998_078_858,
        source_type: SourceType::Linguist,
        name: "Jolie",
        extensions: &["iol", "ol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
