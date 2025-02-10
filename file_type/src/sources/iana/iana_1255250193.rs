use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1255250193: FileType = FileType {
    file_format: &FileFormat {
        id: 1_255_250_193,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcdata-msgstore-ctrl-request+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
