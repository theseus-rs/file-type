use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3778167498: FileType = FileType {
    file_format: &FileFormat {
        id: 3_778_167_498,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcs-location-user-config+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcs-location-user-config+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
