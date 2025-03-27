use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50604600: FileType = FileType {
    file_format: &FileFormat {
        id: 50_604_600,
        source_type: SourceType::Wikidata,
        name: "Phase One Raw Image",
        extensions: &["cap", "capture"],
        media_types: &["image/x-raw-phaseone"],
        signatures: &[],
        related_formats: &[],
    },
};
