use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4228335605: FileType = FileType {
    file_format: &FileFormat {
        id: 4_228_335_605,
        source_type: SourceType::Iana,
        name: "vnd.osgi.dp",
        extensions: &[],
        media_types: &["application/vnd.osgi.dp"],
        signatures: &[],
        related_formats: &[],
    },
};
