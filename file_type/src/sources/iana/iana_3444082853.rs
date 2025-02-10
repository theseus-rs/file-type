use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3444082853: FileType = FileType {
    file_format: &FileFormat {
        id: 3_444_082_853,
        source_type: SourceType::Iana,
        name: "poc-settings+xml",
        extensions: &[],
        media_types: &["application/poc-settings+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
