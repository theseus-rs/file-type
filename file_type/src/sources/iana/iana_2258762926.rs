use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2258762926: FileType = FileType {
    file_format: &FileFormat {
        id: 2_258_762_926,
        source_type: SourceType::Iana,
        name: "vnd.ims.lti.v2.toolproxy+json",
        extensions: &[],
        media_types: &["application/vnd.ims.lti.v2.toolproxy+json"],
        signatures: &[],
        related_formats: &[],
    },
};
