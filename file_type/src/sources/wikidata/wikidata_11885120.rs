use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_11885120: FileType = FileType {
    file_format: &FileFormat {
        id: 11_885_120,
        source_type: SourceType::Wikidata,
        name: "Vorbis",
        extensions: &["oga", "ogg", "sb0"],
        media_types: &[
            "application/ogg",
            "audio/ogg",
            "audio/vorbis",
            "audio/vorbis-config",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
