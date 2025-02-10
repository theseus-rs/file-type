use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2755533877: FileType = FileType {
    file_format: &FileFormat {
        id: 2_755_533_877,
        source_type: SourceType::Iana,
        name: "vnd.oipf.userprofile+xml",
        extensions: &[],
        media_types: &["application/vnd.oipf.userprofile+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
