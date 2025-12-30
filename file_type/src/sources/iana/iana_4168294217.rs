use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4168294217: FileType = FileType {
    file_format: &FileFormat {
        id: 4_168_294_217,
        source_type: SourceType::Iana,
        name: "vec-package+gzip",
        extensions: &[],
        media_types: &["application/vec-package+gzip"],
        signatures: &[],
        related_formats: &[],
    },
};
