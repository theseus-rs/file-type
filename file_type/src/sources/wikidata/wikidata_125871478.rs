use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125871478: FileType = FileType {
    file_format: &FileFormat {
        id: 125_871_478,
        source_type: SourceType::Wikidata,
        name: "PechaMaker Format",
        extensions: &["pxp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
