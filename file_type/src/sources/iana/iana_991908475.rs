use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_991908475: FileType = FileType {
    file_format: &FileFormat {
        id: 991_908_475,
        source_type: SourceType::Iana,
        name: "vnd.zoho.spreadsheetml.sheet",
        extensions: &[],
        media_types: &["application/vnd.zoho.spreadsheetml.sheet"],
        signatures: &[],
        related_formats: &[],
    },
};
