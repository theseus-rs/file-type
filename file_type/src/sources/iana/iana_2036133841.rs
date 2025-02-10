use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2036133841: FileType = FileType {
    file_format: &FileFormat {
        id: 2_036_133_841,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.GMOP+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.GMOP+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
