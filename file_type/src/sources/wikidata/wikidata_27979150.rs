use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979150: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_150,
        source_type: SourceType::Wikidata,
        name: "AN2",
        extensions: &["an2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
