use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_75540713: FileType = FileType {
    file_format: &FileFormat {
        id: 75_540_713,
        source_type: SourceType::Wikidata,
        name: "Ulead PhotoImpact Object",
        extensions: &["ufo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
