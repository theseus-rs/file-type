use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757910: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_910,
        source_type: SourceType::Wikidata,
        name: "Google Document",
        extensions: &["gdoc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
