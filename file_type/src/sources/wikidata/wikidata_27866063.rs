use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27866063: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_063,
        source_type: SourceType::Wikidata,
        name: "Digital Negative, version 1.4",
        extensions: &["dng", "tif"],
        media_types: &["image/dng"],
        signatures: &[],
        related_formats: &[],
    },
};
