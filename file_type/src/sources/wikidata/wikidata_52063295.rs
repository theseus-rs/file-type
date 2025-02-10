use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52063295: FileType = FileType {
    file_format: &FileFormat {
        id: 52_063_295,
        source_type: SourceType::Wikidata,
        name: "SAS for MS-DOS Database",
        extensions: &["ssd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
