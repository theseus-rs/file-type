use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_132245930: FileType = FileType {
    file_format: &FileFormat {
        id: 132_245_930,
        source_type: SourceType::Iana,
        name: "vnd.verimatrix.vcas",
        extensions: &[],
        media_types: &["application/vnd.verimatrix.vcas"],
        signatures: &[],
        related_formats: &[],
    },
};
