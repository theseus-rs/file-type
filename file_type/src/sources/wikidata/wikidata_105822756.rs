use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_105822756: FileType = FileType {
    file_format: &FileFormat {
        id: 105_822_756,
        source_type: SourceType::Wikidata,
        name: "AMDIS RI Calibration Data",
        extensions: &["CAL"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
