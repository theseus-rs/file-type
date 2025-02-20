use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_243018749: FileType = FileType {
    file_format: &FileFormat {
        id: 243_018_749,
        source_type: SourceType::Iana,
        name: "coap-payload",
        extensions: &[],
        media_types: &["application/coap-payload"],
        signatures: &[],
        related_formats: &[],
    },
};
