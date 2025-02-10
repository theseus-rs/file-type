use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_296496: FileType = FileType {
    file_format: &FileFormat {
        id: 296_496,
        source_type: SourceType::Wikidata,
        name: "ARC",
        extensions: &["arc", "ark", "sue"],
        media_types: &["application/x-arc"],
        signatures: &[],
        related_formats: &[],
    },
};
