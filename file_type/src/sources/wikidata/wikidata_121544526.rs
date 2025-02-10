use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121544526: FileType = FileType {
    file_format: &FileFormat {
        id: 121_544_526,
        source_type: SourceType::Wikidata,
        name: "At Home 2010 Tax Return File",
        extensions: &["t10"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
