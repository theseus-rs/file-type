use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112944074: FileType = FileType {
    file_format: &FileFormat {
        id: 112_944_074,
        source_type: SourceType::Wikidata,
        name: "GameExchange2 skeleton file",
        extensions: &["GSF"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
