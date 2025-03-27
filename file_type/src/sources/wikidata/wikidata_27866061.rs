use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27866061: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_061,
        source_type: SourceType::Wikidata,
        name: "Digital Negative, version 1.2",
        extensions: &["dng"],
        media_types: &["image/dng"],
        signatures: &[],
        related_formats: &[],
    },
};
