use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119140819: FileType = FileType {
    file_format: &FileFormat {
        id: 119_140_819,
        source_type: SourceType::Wikidata,
        name: "FreeHand Template 9",
        extensions: &["ft9"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
