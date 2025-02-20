use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28445595: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_595,
        source_type: SourceType::Wikidata,
        name: "Application Object Index",
        extensions: &["axc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
