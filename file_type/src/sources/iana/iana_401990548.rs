use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_401990548: FileType = FileType {
    file_format: &FileFormat {
        id: 401_990_548,
        source_type: SourceType::Iana,
        name: "vnd.wv.ssp+xml",
        extensions: &[],
        media_types: &["application/vnd.wv.ssp+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
