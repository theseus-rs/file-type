use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3228159214: FileType = FileType {
    file_format: &FileFormat {
        id: 3_228_159_214,
        source_type: SourceType::Iana,
        name: "coap-eap",
        extensions: &[],
        media_types: &["application/coap-eap"],
        signatures: &[],
        related_formats: &[],
    },
};
