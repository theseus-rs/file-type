use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1999796314: FileType = FileType {
    file_format: &FileFormat {
        id: 1_999_796_314,
        source_type: SourceType::Iana,
        name: "vnd.sigrok.session",
        extensions: &[],
        media_types: &["application/vnd.sigrok.session"],
        signatures: &[],
        related_formats: &[],
    },
};
