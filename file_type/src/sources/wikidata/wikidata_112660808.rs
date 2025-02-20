use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112660808: FileType = FileType {
    file_format: &FileFormat {
        id: 112_660_808,
        source_type: SourceType::Wikidata,
        name: "MediaView file",
        extensions: &["m14"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
