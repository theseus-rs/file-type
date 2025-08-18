use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2078: FileType = FileType {
    file_format: &FileFormat {
        id: 2_078,
        source_type: SourceType::Wikidata,
        name: "Q2078",
        extensions: &["svg", "svgz"],
        media_types: &["image/SVG", "image/svg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
