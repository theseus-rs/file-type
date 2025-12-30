use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_287315: FileType = FileType {
    file_format: &FileFormat {
        id: 287_315,
        source_type: SourceType::Wikidata,
        name: "reStructuredText",
        extensions: &[],
        media_types: &["text/x-rst"],
        signatures: &[],
        related_formats: &[],
    },
};
