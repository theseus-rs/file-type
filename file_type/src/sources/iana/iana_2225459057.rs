use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2225459057: FileType = FileType {
    file_format: &FileFormat {
        id: 2_225_459_057,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.viewProps+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
