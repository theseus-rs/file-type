use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_311323204: FileType = FileType {
    file_format: &FileFormat {
        id: 311_323_204,
        source_type: SourceType::Iana,
        name: "vnd.nokia.iptv.config+xml",
        extensions: &[],
        media_types: &["application/vnd.nokia.iptv.config+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
