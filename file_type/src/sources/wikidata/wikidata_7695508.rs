use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7695508: FileType = FileType {
    file_format: &FileFormat {
        id: 7_695_508,
        source_type: SourceType::Wikidata,
        name: "Tektronix extended HEX",
        extensions: &["tek"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
