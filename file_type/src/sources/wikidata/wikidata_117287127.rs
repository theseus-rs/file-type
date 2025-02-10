use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117287127: FileType = FileType {
    file_format: &FileFormat {
        id: 117_287_127,
        source_type: SourceType::Wikidata,
        name: "SigmaPlot 1.0, 2.0 Worksheet",
        extensions: &["spw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
