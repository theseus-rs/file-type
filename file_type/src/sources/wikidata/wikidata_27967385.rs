use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967385: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_385,
        source_type: SourceType::Wikidata,
        name: "Extended MIDI",
        extensions: &["xmi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
