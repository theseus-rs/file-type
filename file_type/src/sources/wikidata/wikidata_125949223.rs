use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125949223: FileType = FileType {
    file_format: &FileFormat {
        id: 125_949_223,
        source_type: SourceType::Wikidata,
        name: "ICC Profile iccMAX",
        extensions: &["icc", "icm"],
        media_types: &["application/vnd.iccprofile"],
        signatures: &[],
        related_formats: &[],
    },
};
