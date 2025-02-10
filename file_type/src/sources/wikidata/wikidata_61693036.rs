use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61693036: FileType = FileType {
    file_format: &FileFormat {
        id: 61_693_036,
        source_type: SourceType::Wikidata,
        name: "Waveform Audio",
        extensions: &["wav"],
        media_types: &["audio/wav", "audio/wave", "audio/x-pn-wav", "audio/x-wav"],
        signatures: &[],
        related_formats: &[],
    },
};
