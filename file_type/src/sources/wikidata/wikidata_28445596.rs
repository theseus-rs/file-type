use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28445596: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_596,
        source_type: SourceType::Wikidata,
        name: "APD",
        extensions: &["apd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
