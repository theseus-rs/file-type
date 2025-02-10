use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_257854243: FileType = FileType {
    file_format: &FileFormat {
        id: 257_854_243,
        source_type: SourceType::Iana,
        name: "rpki-publication",
        extensions: &[],
        media_types: &["application/rpki-publication"],
        signatures: &[],
        related_formats: &[],
    },
};
