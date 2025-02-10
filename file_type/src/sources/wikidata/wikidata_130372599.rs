use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130372599: FileType = FileType {
    file_format: &FileFormat {
        id: 130_372_599,
        source_type: SourceType::Wikidata,
        name: "NestedText file format",
        extensions: &["nt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
