use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136697861: FileType = FileType {
    file_format: &FileFormat {
        id: 136_697_861,
        source_type: SourceType::Wikidata,
        name: "MIDI Instrument Definition File",
        extensions: &["idf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
