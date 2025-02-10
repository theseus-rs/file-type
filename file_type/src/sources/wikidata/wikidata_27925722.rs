use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27925722: FileType = FileType {
    file_format: &FileFormat {
        id: 27_925_722,
        source_type: SourceType::Wikidata,
        name: "DTED Level 1 Gazetteer Directory file",
        extensions: &["dir"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
