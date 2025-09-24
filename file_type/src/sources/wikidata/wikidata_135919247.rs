use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135919247: FileType = FileType {
    file_format: &FileFormat {
        id: 135_919_247,
        source_type: SourceType::Wikidata,
        name: "MainActor Project File",
        extensions: &["mpf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
