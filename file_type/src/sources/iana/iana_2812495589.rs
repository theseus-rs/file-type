use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2812495589: FileType = FileType {
    file_format: &FileFormat {
        id: 2_812_495_589,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.seal-location-info+cbor",
        extensions: &[],
        media_types: &["application/vnd.3gpp.seal-location-info+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
