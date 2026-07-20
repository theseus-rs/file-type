use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138427158: FileType = FileType {
    file_format: &FileFormat {
        id: 138_427_158,
        source_type: SourceType::Wikidata,
        name: "Common Ground Digital Paper 3",
        extensions: &["dp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
