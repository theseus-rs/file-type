use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2457510778: FileType = FileType {
    file_format: &FileFormat {
        id: 2_457_510_778,
        source_type: SourceType::Iana,
        name: "prs.implied-object+json-seq",
        extensions: &[],
        media_types: &["application/prs.implied-object+json-seq"],
        signatures: &[],
        related_formats: &[],
    },
};
