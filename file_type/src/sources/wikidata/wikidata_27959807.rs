use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27959807: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_807,
        source_type: SourceType::Wikidata,
        name: "Ableton Live Pack",
        extensions: &["alp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
