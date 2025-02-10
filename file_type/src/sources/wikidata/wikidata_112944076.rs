use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112944076: FileType = FileType {
    file_format: &FileFormat {
        id: 112_944_076,
        source_type: SourceType::Wikidata,
        name: "GameExchange2 lights file",
        extensions: &["GLF"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
