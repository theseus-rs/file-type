use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4067341931: FileType = FileType {
    file_format: &FileFormat {
        id: 4_067_341_931,
        source_type: SourceType::Iana,
        name: "vnd.ldev.productlicensing",
        extensions: &[],
        media_types: &["application/vnd.ldev.productlicensing"],
        signatures: &[],
        related_formats: &[],
    },
};
