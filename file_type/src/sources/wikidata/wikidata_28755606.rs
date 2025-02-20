use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28755606: FileType = FileType {
    file_format: &FileFormat {
        id: 28_755_606,
        source_type: SourceType::Wikidata,
        name: "Exact Yearbook DAT file",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
