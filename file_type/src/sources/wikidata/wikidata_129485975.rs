use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129485975: FileType = FileType {
    file_format: &FileFormat {
        id: 129_485_975,
        source_type: SourceType::Wikidata,
        name: "GraphQL file format",
        extensions: &["graphql"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
