use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3202991632: FileType = FileType {
    file_format: &FileFormat {
        id: 3_202_991_632,
        source_type: SourceType::Iana,
        name: "vnd.balsamiq.bmml+xml",
        extensions: &[],
        media_types: &["application/vnd.balsamiq.bmml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
