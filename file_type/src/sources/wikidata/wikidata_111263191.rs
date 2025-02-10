use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111263191: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_191,
        source_type: SourceType::Wikidata,
        name: "Audio CD compatible raw data",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
