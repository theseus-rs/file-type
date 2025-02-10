use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2034259047: FileType = FileType {
    file_format: &FileFormat {
        id: 2_034_259_047,
        source_type: SourceType::Iana,
        name: "vnd.oipf.mippvcontrolmessage+xml",
        extensions: &[],
        media_types: &["application/vnd.oipf.mippvcontrolmessage+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
