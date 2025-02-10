use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_456286408: FileType = FileType {
    file_format: &FileFormat {
        id: 456_286_408,
        source_type: SourceType::Iana,
        name: "route-s-tsid+xml",
        extensions: &[],
        media_types: &["application/route-s-tsid+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
