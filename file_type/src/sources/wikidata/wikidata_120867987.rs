use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120867987: FileType = FileType {
    file_format: &FileFormat {
        id: 120_867_987,
        source_type: SourceType::Wikidata,
        name: "Cumulus Catalog File",
        extensions: &["ccf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
