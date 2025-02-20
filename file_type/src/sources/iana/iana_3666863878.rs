use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3666863878: FileType = FileType {
    file_format: &FileFormat {
        id: 3_666_863_878,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
