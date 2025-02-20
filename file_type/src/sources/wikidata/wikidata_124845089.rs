use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_124845089: FileType = FileType {
    file_format: &FileFormat {
        id: 124_845_089,
        source_type: SourceType::Wikidata,
        name: "mh",
        extensions: &["mh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
