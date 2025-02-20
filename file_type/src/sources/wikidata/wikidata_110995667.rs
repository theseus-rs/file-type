use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110995667: FileType = FileType {
    file_format: &FileFormat {
        id: 110_995_667,
        source_type: SourceType::Wikidata,
        name: "VideoWave Scene",
        extensions: &["scn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
