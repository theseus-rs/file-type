use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206378: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_378,
        source_type: SourceType::Wikidata,
        name: "IPI",
        extensions: &["ipi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
