use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100136955: FileType = FileType {
    file_format: &FileFormat {
        id: 100_136_955,
        source_type: SourceType::Wikidata,
        name: "XDOMEA 2.3.0",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
