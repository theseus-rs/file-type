use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207108: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_108,
        source_type: SourceType::Wikidata,
        name: "The Print Shop Graphics file",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
