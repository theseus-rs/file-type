use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2461764934: FileType = FileType {
    file_format: &FileFormat {
        id: 2_461_764_934,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.slideLayout+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
