use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
