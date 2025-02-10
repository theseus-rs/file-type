use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_90407344: FileType = FileType {
    file_format: &FileFormat {
        id: 90_407_344,
        source_type: SourceType::Wikidata,
        name: "Strand SSF",
        extensions: &["ssf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
