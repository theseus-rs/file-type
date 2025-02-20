use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121065979: FileType = FileType {
    file_format: &FileFormat {
        id: 121_065_979,
        source_type: SourceType::Wikidata,
        name: "Wizard Database",
        extensions: &["mdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
