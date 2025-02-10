use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_580255865: FileType = FileType {
    file_format: &FileFormat {
        id: 580_255_865,
        source_type: SourceType::Iana,
        name: "prs.fallenstein.rst",
        extensions: &[],
        media_types: &["text/prs.fallenstein.rst"],
        signatures: &[],
        related_formats: &[],
    },
};
