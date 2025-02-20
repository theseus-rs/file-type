use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112819385: FileType = FileType {
    file_format: &FileFormat {
        id: 112_819_385,
        source_type: SourceType::Wikidata,
        name: "Alias TRIangle file",
        extensions: &["tri"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
