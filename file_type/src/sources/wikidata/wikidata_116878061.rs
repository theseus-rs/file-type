use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116878061: FileType = FileType {
    file_format: &FileFormat {
        id: 116_878_061,
        source_type: SourceType::Wikidata,
        name: "Calendar Creator CSV Event File",
        extensions: &["csv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
