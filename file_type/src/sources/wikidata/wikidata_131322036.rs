use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131322036: FileType = FileType {
    file_format: &FileFormat {
        id: 131_322_036,
        source_type: SourceType::Wikidata,
        name: "Treetop file format",
        extensions: &["treetop", "tt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
