use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4172189399: FileType = FileType {
    file_format: &FileFormat {
        id: 4_172_189_399,
        source_type: SourceType::Iana,
        name: "vnd.3gpp2.bcmcsinfo+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp2.bcmcsinfo+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
