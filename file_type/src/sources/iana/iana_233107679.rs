use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_233107679: FileType = FileType {
    file_format: &FileFormat {
        id: 233_107_679,
        source_type: SourceType::Iana,
        name: "vnd.avalon+json",
        extensions: &[],
        media_types: &["application/vnd.avalon+json"],
        signatures: &[],
        related_formats: &[],
    },
};
