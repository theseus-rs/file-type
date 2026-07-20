use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60776453: FileType = FileType {
    file_format: &FileFormat {
        id: 60_776_453,
        source_type: SourceType::Wikidata,
        name: "JPEG XS",
        extensions: &[],
        media_types: &[
            "image/jxs",
            "image/jxsc",
            "image/jxsi",
            "image/jxss",
            "video/jxsv",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
