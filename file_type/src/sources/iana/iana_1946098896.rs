use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1946098896: FileType = FileType {
    file_format: &FileFormat {
        id: 1_946_098_896,
        source_type: SourceType::Iana,
        name: "vnd.yamaha.hv-script",
        extensions: &[],
        media_types: &["application/vnd.yamaha.hv-script"],
        signatures: &[],
        related_formats: &[],
    },
};
