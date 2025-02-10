use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_303387740: FileType = FileType {
    file_format: &FileFormat {
        id: 303_387_740,
        source_type: SourceType::Iana,
        name: "multipart-core",
        extensions: &[],
        media_types: &["application/multipart-core"],
        signatures: &[],
        related_formats: &[],
    },
};
