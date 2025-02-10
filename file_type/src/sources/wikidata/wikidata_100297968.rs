use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100297968: FileType = FileType {
    file_format: &FileFormat {
        id: 100_297_968,
        source_type: SourceType::Wikidata,
        name: "Flow Charting file format, version 4",
        extensions: &["gfc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
