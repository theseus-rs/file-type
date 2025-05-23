use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4104042877: FileType = FileType {
    file_format: &FileFormat {
        id: 4_104_042_877,
        source_type: SourceType::Iana,
        name: "davmount+xml",
        extensions: &[],
        media_types: &["application/davmount+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
