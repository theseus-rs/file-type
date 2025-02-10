use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60414423: FileType = FileType {
    file_format: &FileFormat {
        id: 60_414_423,
        source_type: SourceType::Wikidata,
        name: "TAP (Commodore 64)",
        extensions: &["tap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
