use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2195732661: FileType = FileType {
    file_format: &FileFormat {
        id: 2_195_732_661,
        source_type: SourceType::Iana,
        name: "gnap-binding-rotation-jwsd",
        extensions: &[],
        media_types: &["application/gnap-binding-rotation-jwsd"],
        signatures: &[],
        related_formats: &[],
    },
};
