use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110946240: FileType = FileType {
    file_format: &FileFormat {
        id: 110_946_240,
        source_type: SourceType::Wikidata,
        name: "Drools Rule Language",
        extensions: &["drl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
