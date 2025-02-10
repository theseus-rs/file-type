use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50223921: FileType = FileType {
    file_format: &FileFormat {
        id: 50_223_921,
        source_type: SourceType::Wikidata,
        name: "Adobe Air",
        extensions: &["air"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
