use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28915683: FileType = FileType {
    file_format: &FileFormat {
        id: 28_915_683,
        source_type: SourceType::Wikidata,
        name: "Apache Parquet",
        extensions: &["parquet"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
