use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206147: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_147,
        source_type: SourceType::Wikidata,
        name: "Freedom of Press Bitamp",
        extensions: &["1", "fop"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
