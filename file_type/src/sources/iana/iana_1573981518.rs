use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1573981518: FileType = FileType {
    file_format: &FileFormat {
        id: 1_573_981_518,
        source_type: SourceType::Iana,
        name: "private-token-response",
        extensions: &[],
        media_types: &["application/private-token-response"],
        signatures: &[],
        related_formats: &[],
    },
};
