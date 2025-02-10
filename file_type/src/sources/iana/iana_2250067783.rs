use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2250067783: FileType = FileType {
    file_format: &FileFormat {
        id: 2_250_067_783,
        source_type: SourceType::Iana,
        name: "vnd.oipf.dae.svg+xml",
        extensions: &[],
        media_types: &["application/vnd.oipf.dae.svg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
