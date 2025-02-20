use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_591863795: FileType = FileType {
    file_format: &FileFormat {
        id: 591_863_795,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.template",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.template"],
        signatures: &[],
        related_formats: &[],
    },
};
