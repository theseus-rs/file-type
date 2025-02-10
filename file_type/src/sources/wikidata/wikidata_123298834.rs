use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123298834: FileType = FileType {
    file_format: &FileFormat {
        id: 123_298_834,
        source_type: SourceType::Wikidata,
        name: "Retrospect UTX File",
        extensions: &["utx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
