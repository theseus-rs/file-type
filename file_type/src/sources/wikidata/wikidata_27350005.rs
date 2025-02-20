use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27350005: FileType = FileType {
    file_format: &FileFormat {
        id: 27_350_005,
        source_type: SourceType::Wikidata,
        name: "TOPSAR L-Band Polarimetry Data",
        extensions: &["datgr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
