use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135401190: FileType = FileType {
    file_format: &FileFormat {
        id: 135_401_190,
        source_type: SourceType::Wikidata,
        name: "Rake file",
        extensions: &["rake"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
