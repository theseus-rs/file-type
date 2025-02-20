use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_537661535: FileType = FileType {
    file_format: &FileFormat {
        id: 537_661_535,
        source_type: SourceType::Iana,
        name: "vnd.ipfs.ipns-record",
        extensions: &[],
        media_types: &["application/vnd.ipfs.ipns-record"],
        signatures: &[],
        related_formats: &[],
    },
};
