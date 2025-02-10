use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_783502289: FileType = FileType {
    file_format: &FileFormat {
        id: 783_502_289,
        source_type: SourceType::Iana,
        name: "index.cmd",
        extensions: &[],
        media_types: &["application/index.cmd"],
        signatures: &[],
        related_formats: &[],
    },
};
