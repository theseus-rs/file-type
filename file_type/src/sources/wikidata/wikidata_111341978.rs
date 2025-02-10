use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111341978: FileType = FileType {
    file_format: &FileFormat {
        id: 111_341_978,
        source_type: SourceType::Wikidata,
        name: "MIDI Converter Studio packed Sound Font",
        extensions: &["sf2pack"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
