use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117600048: FileType = FileType {
    file_format: &FileFormat {
        id: 117_600_048,
        source_type: SourceType::Wikidata,
        name: "Digital Negative, version 1.5",
        extensions: &["dng"],
        media_types: &["image/tiff"],
        signatures: &[],
        related_formats: &[],
    },
};
