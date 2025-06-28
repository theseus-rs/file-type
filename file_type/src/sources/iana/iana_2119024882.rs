use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2119024882: FileType = FileType {
    file_format: &FileFormat {
        id: 2_119_024_882,
        source_type: SourceType::Iana,
        name: "vnd.oms.cellular-cose-content+cbor",
        extensions: &[],
        media_types: &["application/vnd.oms.cellular-cose-content+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
