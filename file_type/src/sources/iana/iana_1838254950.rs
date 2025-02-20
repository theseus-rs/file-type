use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1838254950: FileType = FileType {
    file_format: &FileFormat {
        id: 1_838_254_950,
        source_type: SourceType::Iana,
        name: "vnd.oipf.ueprofile+xml",
        extensions: &[],
        media_types: &["application/vnd.oipf.ueprofile+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
