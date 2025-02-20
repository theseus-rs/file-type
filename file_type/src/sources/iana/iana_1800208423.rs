use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1800208423: FileType = FileType {
    file_format: &FileFormat {
        id: 1_800_208_423,
        source_type: SourceType::Iana,
        name: "vnd.gpxsee.map+xml",
        extensions: &[],
        media_types: &["application/vnd.gpxsee.map+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
