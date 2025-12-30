use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3318246417: FileType = FileType {
    file_format: &FileFormat {
        id: 3_318_246_417,
        source_type: SourceType::Iana,
        name: "vnd.uic.dosipas.v1",
        extensions: &[],
        media_types: &["application/vnd.uic.dosipas.v1"],
        signatures: &[],
        related_formats: &[],
    },
};
