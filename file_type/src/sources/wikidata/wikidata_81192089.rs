use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_81192089: FileType = FileType {
    file_format: &FileFormat {
        id: 81_192_089,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine Compiled Script",
        extensions: &["bcs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
