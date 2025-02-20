use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4248394186: FileType = FileType {
    file_format: &FileFormat {
        id: 4_248_394_186,
        source_type: SourceType::Iana,
        name: "gnap-binding-jwsd",
        extensions: &[],
        media_types: &["application/gnap-binding-jwsd"],
        signatures: &[],
        related_formats: &[],
    },
};
