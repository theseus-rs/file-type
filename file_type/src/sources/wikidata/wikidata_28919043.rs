use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28919043: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_043,
        source_type: SourceType::Wikidata,
        name: "Sony HDV",
        extensions: &["m2t"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
