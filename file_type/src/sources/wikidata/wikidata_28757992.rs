use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757992: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_992,
        source_type: SourceType::Wikidata,
        name: "ISZ",
        extensions: &["isz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
