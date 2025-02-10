use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125255794: FileType = FileType {
    file_format: &FileFormat {
        id: 125_255_794,
        source_type: SourceType::Wikidata,
        name: "CombiTimeTable",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
