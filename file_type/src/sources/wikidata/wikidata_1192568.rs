use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1192568: FileType = FileType {
    file_format: &FileFormat {
        id: 1_192_568,
        source_type: SourceType::Wikidata,
        name: ".sys",
        extensions: &["sys"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
