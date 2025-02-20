use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123298805: FileType = FileType {
    file_format: &FileFormat {
        id: 123_298_805,
        source_type: SourceType::Wikidata,
        name: "Retrospect RDX File",
        extensions: &["rdx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
