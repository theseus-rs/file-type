use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136555964: FileType = FileType {
    file_format: &FileFormat {
        id: 136_555_964,
        source_type: SourceType::Wikidata,
        name: "Citrus Card Image",
        extensions: &["3ds", "cci"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
