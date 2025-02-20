use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_73160161: FileType = FileType {
    file_format: &FileFormat {
        id: 73_160_161,
        source_type: SourceType::Wikidata,
        name: "Visio Drawing Template",
        extensions: &["vst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
