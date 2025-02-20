use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3736746635: FileType = FileType {
    file_format: &FileFormat {
        id: 3_736_746_635,
        source_type: SourceType::Iana,
        name: "prs.implied-object+json",
        extensions: &[],
        media_types: &["application/prs.implied-object+json"],
        signatures: &[],
        related_formats: &[],
    },
};
