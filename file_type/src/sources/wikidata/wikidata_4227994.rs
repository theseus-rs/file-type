use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4227994: FileType = FileType {
    file_format: &FileFormat {
        id: 4_227_994,
        source_type: SourceType::Wikidata,
        name: "Shareaza collection",
        extensions: &["co"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
