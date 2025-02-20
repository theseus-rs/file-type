use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113365166: FileType = FileType {
    file_format: &FileFormat {
        id: 113_365_166,
        source_type: SourceType::Wikidata,
        name: "Camtasia Recording",
        extensions: &["trec"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
