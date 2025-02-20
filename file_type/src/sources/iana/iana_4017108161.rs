use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4017108161: FileType = FileType {
    file_format: &FileFormat {
        id: 4_017_108_161,
        source_type: SourceType::Iana,
        name: "swid+cbor",
        extensions: &[],
        media_types: &["application/swid+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
