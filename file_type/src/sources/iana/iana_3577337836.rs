use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3577337836: FileType = FileType {
    file_format: &FileFormat {
        id: 3_577_337_836,
        source_type: SourceType::Iana,
        name: "protobuf",
        extensions: &[],
        media_types: &["application/protobuf"],
        signatures: &[],
        related_formats: &[],
    },
};
