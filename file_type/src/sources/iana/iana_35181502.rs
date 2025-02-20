use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_35181502: FileType = FileType {
    file_format: &FileFormat {
        id: 35_181_502,
        source_type: SourceType::Iana,
        name: "vnd.century-systems.tcp_stream",
        extensions: &[],
        media_types: &["application/vnd.century-systems.tcp_stream"],
        signatures: &[],
        related_formats: &[],
    },
};
