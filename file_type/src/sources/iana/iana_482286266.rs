use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_482286266: FileType = FileType {
    file_format: &FileFormat {
        id: 482_286_266,
        source_type: SourceType::Iana,
        name: "marcxml+xml",
        extensions: &[],
        media_types: &["application/marcxml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
