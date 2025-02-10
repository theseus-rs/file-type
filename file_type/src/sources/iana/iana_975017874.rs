use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_975017874: FileType = FileType {
    file_format: &FileFormat {
        id: 975_017_874,
        source_type: SourceType::Iana,
        name: "vnd.sealed.csf",
        extensions: &[],
        media_types: &["application/vnd.sealed.csf"],
        signatures: &[],
        related_formats: &[],
    },
};
