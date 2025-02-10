use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110995712: FileType = FileType {
    file_format: &FileFormat {
        id: 110_995_712,
        source_type: SourceType::Wikidata,
        name: "VideoWave Production File",
        extensions: &["sbd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
