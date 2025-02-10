use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206090: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_090,
        source_type: SourceType::Wikidata,
        name: "TT Low Resolution",
        extensions: &["PI8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
