use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_73160398: FileType = FileType {
    file_format: &FileFormat {
        id: 73_160_398,
        source_type: SourceType::Wikidata,
        name: "Visio Stencil",
        extensions: &["vss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
