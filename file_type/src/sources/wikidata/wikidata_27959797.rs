use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27959797: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_797,
        source_type: SourceType::Wikidata,
        name: "Ableton Device Preset",
        extensions: &["adp", "adv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
