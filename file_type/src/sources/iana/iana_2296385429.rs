use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2296385429: FileType = FileType {
    file_format: &FileFormat {
        id: 2_296_385_429,
        source_type: SourceType::Iana,
        name: "vnd.irepository.package+xml",
        extensions: &[],
        media_types: &["application/vnd.irepository.package+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
