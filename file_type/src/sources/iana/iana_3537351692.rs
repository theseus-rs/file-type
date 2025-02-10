use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3537351692: FileType = FileType {
    file_format: &FileFormat {
        id: 3_537_351_692,
        source_type: SourceType::Iana,
        name: "vnd.dece.mobile",
        extensions: &[],
        media_types: &["video/vnd.dece.mobile"],
        signatures: &[],
        related_formats: &[],
    },
};
