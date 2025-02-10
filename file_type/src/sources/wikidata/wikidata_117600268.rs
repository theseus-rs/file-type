use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117600268: FileType = FileType {
    file_format: &FileFormat {
        id: 117_600_268,
        source_type: SourceType::Wikidata,
        name: "Digital Negative, version 1.6",
        extensions: &["dng", "tif"],
        media_types: &["image/tiff"],
        signatures: &[],
        related_formats: &[],
    },
};
