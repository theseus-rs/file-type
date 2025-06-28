use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3841339908: FileType = FileType {
    file_format: &FileFormat {
        id: 3_841_339_908,
        source_type: SourceType::Iana,
        name: "vnd.pyon+json",
        extensions: &[],
        media_types: &["application/vnd.pyon+json"],
        signatures: &[],
        related_formats: &[],
    },
};
