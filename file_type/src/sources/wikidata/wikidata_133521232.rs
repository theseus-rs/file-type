use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133521232: FileType = FileType {
    file_format: &FileFormat {
        id: 133_521_232,
        source_type: SourceType::Wikidata,
        name: "DataShow Graphic",
        extensions: &["gra"],
        media_types: &["image/x-datashow-graphic"],
        signatures: &[],
        related_formats: &[],
    },
};
