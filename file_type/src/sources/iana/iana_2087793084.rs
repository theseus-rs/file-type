use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2087793084: FileType = FileType {
    file_format: &FileFormat {
        id: 2_087_793_084,
        source_type: SourceType::Iana,
        name: "vnd.vtu",
        extensions: &[],
        media_types: &["model/vnd.vtu"],
        signatures: &[],
        related_formats: &[],
    },
};
