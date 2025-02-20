use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110238221: FileType = FileType {
    file_format: &FileFormat {
        id: 110_238_221,
        source_type: SourceType::Wikidata,
        name: "FrameImage",
        extensions: &["fmg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
