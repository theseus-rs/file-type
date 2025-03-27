use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27866059: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_059,
        source_type: SourceType::Wikidata,
        name: "Digital Negative, version 1.1",
        extensions: &["dng", "tif"],
        media_types: &["image/dng"],
        signatures: &[],
        related_formats: &[],
    },
};
