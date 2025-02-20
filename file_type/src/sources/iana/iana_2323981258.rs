use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2323981258: FileType = FileType {
    file_format: &FileFormat {
        id: 2_323_981_258,
        source_type: SourceType::Iana,
        name: "vnd.nokia.pcd+xml",
        extensions: &[],
        media_types: &["application/vnd.nokia.pcd+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
