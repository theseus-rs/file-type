use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105858335: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_335,
        source_type: SourceType::Wikidata,
        name: "Adobe Edge Project",
        extensions: &["edge"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
