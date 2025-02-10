use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2421251805: FileType = FileType {
    file_format: &FileFormat {
        id: 2_421_251_805,
        source_type: SourceType::Iana,
        name: "aif+json",
        extensions: &[],
        media_types: &["application/aif+json"],
        signatures: &[],
        related_formats: &[],
    },
};
