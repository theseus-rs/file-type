use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2196028526: FileType = FileType {
    file_format: &FileFormat {
        id: 2_196_028_526,
        source_type: SourceType::Iana,
        name: "prs.hpub+zip",
        extensions: &[],
        media_types: &["application/prs.hpub+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
