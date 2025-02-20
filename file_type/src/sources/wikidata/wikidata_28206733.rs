use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206733: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_733,
        source_type: SourceType::Wikidata,
        name: "Newsroom",
        extensions: &["nsr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
