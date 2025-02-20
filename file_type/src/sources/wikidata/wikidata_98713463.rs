use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_98713463: FileType = FileType {
    file_format: &FileFormat {
        id: 98_713_463,
        source_type: SourceType::Wikidata,
        name: "POV-Ray",
        extensions: &["pov"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
