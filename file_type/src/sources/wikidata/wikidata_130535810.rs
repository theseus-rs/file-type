use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130535810: FileType = FileType {
    file_format: &FileFormat {
        id: 130_535_810,
        source_type: SourceType::Wikidata,
        name: "PromQL query file format",
        extensions: &["promql"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
