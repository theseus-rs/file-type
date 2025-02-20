use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1133956420: FileType = FileType {
    file_format: &FileFormat {
        id: 1_133_956_420,
        source_type: SourceType::Iana,
        name: "soap+fastinfoset",
        extensions: &[],
        media_types: &["application/soap+fastinfoset"],
        signatures: &[],
        related_formats: &[],
    },
};
