use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29151590: FileType = FileType {
    file_format: &FileFormat {
        id: 29_151_590,
        source_type: SourceType::Wikidata,
        name: "Redcode oBJect",
        extensions: &["rbj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
