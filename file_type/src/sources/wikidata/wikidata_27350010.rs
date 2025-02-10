use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27350010: FileType = FileType {
    file_format: &FileFormat {
        id: 27_350_010,
        source_type: SourceType::Wikidata,
        name: "TOPSAR P-Band Polarimetry Data",
        extensions: &["datgr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
