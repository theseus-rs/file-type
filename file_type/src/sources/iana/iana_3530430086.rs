use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3530430086: FileType = FileType {
    file_format: &FileFormat {
        id: 3_530_430_086,
        source_type: SourceType::Iana,
        name: "vnd.coreos.ignition+json",
        extensions: &[],
        media_types: &["application/vnd.coreos.ignition+json"],
        signatures: &[],
        related_formats: &[],
    },
};
