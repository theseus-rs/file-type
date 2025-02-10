use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967383: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_383,
        source_type: SourceType::Wikidata,
        name: "RIFF MIDI",
        extensions: &["rmi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
