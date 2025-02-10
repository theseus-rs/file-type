use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125297644: FileType = FileType {
    file_format: &FileFormat {
        id: 125_297_644,
        source_type: SourceType::Wikidata,
        name: "Scheme library source",
        extensions: &["sls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
