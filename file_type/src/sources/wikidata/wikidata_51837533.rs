use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51837533: FileType = FileType {
    file_format: &FileFormat {
        id: 51_837_533,
        source_type: SourceType::Wikidata,
        name: "Visual FoxPro Database Container File",
        extensions: &["dcx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
