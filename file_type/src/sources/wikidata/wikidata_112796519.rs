use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112796519: FileType = FileType {
    file_format: &FileFormat {
        id: 112_796_519,
        source_type: SourceType::Wikidata,
        name: "Sony RAW Image",
        extensions: &["srf"],
        media_types: &["image/x-sony-srf"],
        signatures: &[],
        related_formats: &[],
    },
};
