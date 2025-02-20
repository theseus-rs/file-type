use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206493: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_493,
        source_type: SourceType::Wikidata,
        name: "Lightning Strike",
        extensions: &["cod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
