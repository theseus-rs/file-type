use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34747002: FileType = FileType {
    file_format: &FileFormat {
        id: 34_747_002,
        source_type: SourceType::Wikidata,
        name: "STATISTICA Project File",
        extensions: &["spf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
