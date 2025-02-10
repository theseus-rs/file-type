use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2436336104: FileType = FileType {
    file_format: &FileFormat {
        id: 2_436_336_104,
        source_type: SourceType::Iana,
        name: "vnd.onvif.metadata",
        extensions: &[],
        media_types: &["application/vnd.onvif.metadata"],
        signatures: &[],
        related_formats: &[],
    },
};
