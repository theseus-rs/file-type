use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125947579: FileType = FileType {
    file_format: &FileFormat {
        id: 125_947_579,
        source_type: SourceType::Wikidata,
        name: "ICC Profile 2",
        extensions: &["icc", "icm"],
        media_types: &["application/vnd.iccprofile"],
        signatures: &[],
        related_formats: &[],
    },
};
