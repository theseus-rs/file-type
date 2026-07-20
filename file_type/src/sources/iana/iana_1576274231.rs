use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1576274231: FileType = FileType {
    file_format: &FileFormat {
        id: 1_576_274_231,
        source_type: SourceType::Iana,
        name: "vnd.nila.protobuf-bundle+zip",
        extensions: &[],
        media_types: &["application/vnd.nila.protobuf-bundle+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
