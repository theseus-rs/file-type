use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3429254293: FileType = FileType {
    file_format: &FileFormat {
        id: 3_429_254_293,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcvideo-transmission-request+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcvideo-transmission-request+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
