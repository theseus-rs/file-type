use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59914669: FileType = FileType {
    file_format: &FileFormat {
        id: 59_914_669,
        source_type: SourceType::Wikidata,
        name: "Steel Detailing Neutral Format",
        extensions: &["sdn"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
