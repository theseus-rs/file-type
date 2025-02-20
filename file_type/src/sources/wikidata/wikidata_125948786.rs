use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125948786: FileType = FileType {
    file_format: &FileFormat {
        id: 125_948_786,
        source_type: SourceType::Wikidata,
        name: "ICC Profile 4",
        extensions: &["icc", "icm"],
        media_types: &["application/vnd.iccprofile"],
        signatures: &[],
        related_formats: &[],
    },
};
