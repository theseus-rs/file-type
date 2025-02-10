use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111332298: FileType = FileType {
    file_format: &FileFormat {
        id: 111_332_298,
        source_type: SourceType::Wikidata,
        name: "Typhoon voice file",
        extensions: &["o01"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
