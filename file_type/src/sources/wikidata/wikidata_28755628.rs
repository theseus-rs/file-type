use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28755628: FileType = FileType {
    file_format: &FileFormat {
        id: 28_755_628,
        source_type: SourceType::Wikidata,
        name: "Exact Yearbook LST file",
        extensions: &["lst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
