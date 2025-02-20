use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113534253: FileType = FileType {
    file_format: &FileFormat {
        id: 113_534_253,
        source_type: SourceType::Wikidata,
        name: "Geosoft Map Description File",
        extensions: &["mdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
