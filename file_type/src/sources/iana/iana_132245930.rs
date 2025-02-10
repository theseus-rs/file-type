use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
