use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_73160459: FileType = FileType {
    file_format: &FileFormat {
        id: 73_160_459,
        source_type: SourceType::Wikidata,
        name: "Visio Workspace",
        extensions: &["vsw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
