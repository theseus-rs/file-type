use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110501470: FileType = FileType {
    file_format: &FileFormat {
        id: 110_501_470,
        source_type: SourceType::Wikidata,
        name: "reStructuredText format",
        extensions: &["rst"],
        media_types: &["text/x-rst"],
        signatures: &[],
        related_formats: &[],
    },
};
