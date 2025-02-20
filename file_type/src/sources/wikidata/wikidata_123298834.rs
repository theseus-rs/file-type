use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
