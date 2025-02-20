use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111263309: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_309,
        source_type: SourceType::Wikidata,
        name: "Sound Designer I file",
        extensions: &["dig", "sd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
