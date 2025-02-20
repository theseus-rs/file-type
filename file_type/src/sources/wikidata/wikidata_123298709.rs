use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123298709: FileType = FileType {
    file_format: &FileFormat {
        id: 123_298_709,
        source_type: SourceType::Wikidata,
        name: "Retrospect RBC File",
        extensions: &["rbc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
