use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28445582: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_582,
        source_type: SourceType::Wikidata,
        name: "AGSC",
        extensions: &["agsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
