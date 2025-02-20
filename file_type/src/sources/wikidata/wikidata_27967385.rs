use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
