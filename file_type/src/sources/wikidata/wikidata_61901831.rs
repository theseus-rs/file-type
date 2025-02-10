use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61901831: FileType = FileType {
    file_format: &FileFormat {
        id: 61_901_831,
        source_type: SourceType::Wikidata,
        name: "Peak Graphical Waveform File",
        extensions: &["pk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
