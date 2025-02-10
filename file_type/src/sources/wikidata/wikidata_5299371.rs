use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5299371: FileType = FileType {
    file_format: &FileFormat {
        id: 5_299_371,
        source_type: SourceType::Wikidata,
        name: "dotXSI",
        extensions: &["xsi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
