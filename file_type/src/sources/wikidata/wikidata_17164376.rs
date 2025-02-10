use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_17164376: FileType = FileType {
    file_format: &FileFormat {
        id: 17_164_376,
        source_type: SourceType::Wikidata,
        name: "Scalable Vector Graphics Compressed",
        extensions: &["svgz"],
        media_types: &["image/svg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
