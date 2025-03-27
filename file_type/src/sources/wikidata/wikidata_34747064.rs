use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34747064: FileType = FileType {
    file_format: &FileFormat {
        id: 34_747_064,
        source_type: SourceType::Wikidata,
        name: "STATISTICA Data Miners Recipes Project File",
        extensions: &["dmrproj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
