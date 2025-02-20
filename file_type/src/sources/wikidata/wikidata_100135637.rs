use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100135637: FileType = FileType {
    file_format: &FileFormat {
        id: 100_135_637,
        source_type: SourceType::Wikidata,
        name: "XDOMEA 2.1.0",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
