use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111648750: FileType = FileType {
    file_format: &FileFormat {
        id: 111_648_750,
        source_type: SourceType::Wikidata,
        name: "Easy Prints file",
        extensions: &["php"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
