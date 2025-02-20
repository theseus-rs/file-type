use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_52230534: FileType = FileType {
    file_format: &FileFormat {
        id: 52_230_534,
        source_type: SourceType::Wikidata,
        name: "Polynomial Texture Map",
        extensions: &["ptm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
