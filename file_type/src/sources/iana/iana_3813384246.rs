use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3813384246: FileType = FileType {
    file_format: &FileFormat {
        id: 3_813_384_246,
        source_type: SourceType::Iana,
        name: "vnd.aristanetworks.swi",
        extensions: &[],
        media_types: &["application/vnd.aristanetworks.swi"],
        signatures: &[],
        related_formats: &[],
    },
};
