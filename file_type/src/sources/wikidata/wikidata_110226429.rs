use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110226429: FileType = FileType {
    file_format: &FileFormat {
        id: 110_226_429,
        source_type: SourceType::Wikidata,
        name: "ELAN Preference File",
        extensions: &["pfsx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
