use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138427323: FileType = FileType {
    file_format: &FileFormat {
        id: 138_427_323,
        source_type: SourceType::Wikidata,
        name: "Common Ground Digital Paper 4",
        extensions: &["dp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
