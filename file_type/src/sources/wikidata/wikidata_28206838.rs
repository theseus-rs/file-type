use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206838: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_838,
        source_type: SourceType::Wikidata,
        name: "Palm bitmap",
        extensions: &["palm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
