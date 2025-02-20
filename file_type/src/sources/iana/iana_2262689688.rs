use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2262689688: FileType = FileType {
    file_format: &FileFormat {
        id: 2_262_689_688,
        source_type: SourceType::Iana,
        name: "vq-rtcpxr",
        extensions: &[],
        media_types: &["application/vq-rtcpxr"],
        signatures: &[],
        related_formats: &[],
    },
};
