use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100301097: FileType = FileType {
    file_format: &FileFormat {
        id: 100_301_097,
        source_type: SourceType::Wikidata,
        name: "Flow Charting PDQ format",
        extensions: &["pdq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
