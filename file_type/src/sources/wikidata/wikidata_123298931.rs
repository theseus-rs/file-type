use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123298931: FileType = FileType {
    file_format: &FileFormat {
        id: 123_298_931,
        source_type: SourceType::Wikidata,
        name: "Retrospect RRR File",
        extensions: &["rrr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
