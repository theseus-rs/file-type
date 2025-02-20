use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1120915: FileType = FileType {
    file_format: &FileFormat {
        id: 1_120_915,
        source_type: SourceType::Wikidata,
        name: "Compact Disc Audio track",
        extensions: &["cda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
