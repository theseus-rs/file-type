use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_379741: FileType = FileType {
    file_format: &FileFormat {
        id: 379_741,
        source_type: SourceType::Wikidata,
        name: "Advanced Systems Format",
        extensions: &["asf"],
        media_types: &[
            "application/ms-asf",
            "application/vnd.ms-asf",
            "video/x-ms-asf",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
