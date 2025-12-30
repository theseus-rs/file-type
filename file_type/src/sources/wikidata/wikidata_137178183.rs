use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137178183: FileType = FileType {
    file_format: &FileFormat {
        id: 137_178_183,
        source_type: SourceType::Wikidata,
        name: "MySQL MISAM compressed data",
        extensions: &["myi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
