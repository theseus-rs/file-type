use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4141392139: FileType = FileType {
    file_format: &FileFormat {
        id: 4_141_392_139,
        source_type: SourceType::Iana,
        name: "cmw+cose",
        extensions: &[],
        media_types: &["application/cmw+cose"],
        signatures: &[],
        related_formats: &[],
    },
};
