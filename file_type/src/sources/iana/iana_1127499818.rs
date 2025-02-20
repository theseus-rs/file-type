use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1127499818: FileType = FileType {
    file_format: &FileFormat {
        id: 1_127_499_818,
        source_type: SourceType::Iana,
        name: "edhoc+cbor-seq",
        extensions: &[],
        media_types: &["application/edhoc+cbor-seq"],
        signatures: &[],
        related_formats: &[],
    },
};
