use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28921959: FileType = FileType {
    file_format: &FileFormat {
        id: 28_921_959,
        source_type: SourceType::Wikidata,
        name: "Keyhole Markup Language Zipped",
        extensions: &["kmz"],
        media_types: &["application/vnd.google-earth.kmz"],
        signatures: &[],
        related_formats: &[],
    },
};
