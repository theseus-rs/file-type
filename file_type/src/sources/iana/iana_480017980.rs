use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_480017980: FileType = FileType {
    file_format: &FileFormat {
        id: 480_017_980,
        source_type: SourceType::Iana,
        name: "urc-targetdesc+xml",
        extensions: &[],
        media_types: &["application/urc-targetdesc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
