use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2634876555: FileType = FileType {
    file_format: &FileFormat {
        id: 2_634_876_555,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.seal-data-delivery-info+cbor",
        extensions: &[],
        media_types: &["application/vnd.3gpp.seal-data-delivery-info+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
