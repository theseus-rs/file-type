use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52063281: FileType = FileType {
    file_format: &FileFormat {
        id: 52_063_281,
        source_type: SourceType::Wikidata,
        name: "SAS Data File",
        extensions: &["ssd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
