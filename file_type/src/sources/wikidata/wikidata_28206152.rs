use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206152: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_152,
        source_type: SourceType::Wikidata,
        name: "FSH",
        extensions: &["fsh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
