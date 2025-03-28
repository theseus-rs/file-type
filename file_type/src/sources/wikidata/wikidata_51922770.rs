use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51922770: FileType = FileType {
    file_format: &FileFormat {
        id: 51_922_770,
        source_type: SourceType::Wikidata,
        name: "Adobe ACD",
        extensions: &["acd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
