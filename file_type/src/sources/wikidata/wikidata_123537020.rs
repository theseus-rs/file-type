use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123537020: FileType = FileType {
    file_format: &FileFormat {
        id: 123_537_020,
        source_type: SourceType::Wikidata,
        name: "QtiPlot document",
        extensions: &["qti"],
        media_types: &["application/x-qtiplot"],
        signatures: &[],
        related_formats: &[],
    },
};
