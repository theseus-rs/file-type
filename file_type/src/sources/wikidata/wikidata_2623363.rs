use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2623363: FileType = FileType {
    file_format: &FileFormat {
        id: 2_623_363,
        source_type: SourceType::Wikidata,
        name: "ANSI art",
        extensions: &["ans"],
        media_types: &["text/x-ansi"],
        signatures: &[],
        related_formats: &[],
    },
};
