use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1088321248: FileType = FileType {
    file_format: &FileFormat {
        id: 1_088_321_248,
        source_type: SourceType::Iana,
        name: "vnd.openvpi.dspx+json",
        extensions: &[],
        media_types: &["application/vnd.openvpi.dspx+json"],
        signatures: &[],
        related_formats: &[],
    },
};
