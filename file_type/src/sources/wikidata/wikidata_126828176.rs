use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126828176: FileType = FileType {
    file_format: &FileFormat {
        id: 126_828_176,
        source_type: SourceType::Wikidata,
        name: "Forth source code file",
        extensions: &["fs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
