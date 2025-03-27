use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_337594: FileType = FileType {
    file_format: &FileFormat {
        id: 337_594,
        source_type: SourceType::Wikidata,
        name: "Advanced Audio Coding",
        extensions: &["3gp", "aac", "m4a", "m4b", "m4p", "mp4"],
        media_types: &[
            "audio/aac",
            "audio/aacp",
            "audio/mp4a",
            "audio/mp4a-latm",
            "audio/mpeg4-generic",
            "audio/mpga",
            "audio/x-aac",
            "audio/x-m4a",
            "audio/x-m4b",
            "audio/x-m4p",
            "audio/x-mp4a-latm",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
