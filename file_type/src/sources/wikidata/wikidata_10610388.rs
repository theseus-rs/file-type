use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_10610388: FileType = FileType {
    file_format: &FileFormat {
        id: 10_610_388,
        source_type: SourceType::Wikidata,
        name: "Standard MIDI File",
        extensions: &["mid", "midi"],
        media_types: &["audio/midi"],
        signatures: &[],
        related_formats: &[],
    },
};
