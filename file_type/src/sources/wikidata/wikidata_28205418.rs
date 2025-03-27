use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205418: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_418,
        source_type: SourceType::Wikidata,
        name: "Panasonic RAW/RW2",
        extensions: &["raw", "rw1", "rw2"],
        media_types: &[
            "image/x-panasonic-raw",
            "image/x-panasonic-raw2",
            "image/x-panasonic-rw",
            "image/x-panasonic-rw2",
            "image/x-raw-panasonic",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
