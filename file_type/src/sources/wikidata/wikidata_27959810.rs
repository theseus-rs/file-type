use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27959810: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_810,
        source_type: SourceType::Wikidata,
        name: "Ableton Live Set",
        extensions: &["als"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
