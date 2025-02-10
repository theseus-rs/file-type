use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100243992: FileType = FileType {
    file_format: &FileFormat {
        id: 100_243_992,
        source_type: SourceType::Wikidata,
        name: "Student Writing Center Sign",
        extensions: &["sg", "sgt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
