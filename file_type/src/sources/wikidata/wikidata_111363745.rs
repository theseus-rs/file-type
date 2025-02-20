use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111363745: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_745,
        source_type: SourceType::Wikidata,
        name: "Miles Sound System extended MIDI file",
        extensions: &["xmi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
