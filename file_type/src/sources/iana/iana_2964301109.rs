use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2964301109: FileType = FileType {
    file_format: &FileFormat {
        id: 2_964_301_109,
        source_type: SourceType::Iana,
        name: "sieve",
        extensions: &[],
        media_types: &["application/sieve"],
        signatures: &[],
        related_formats: &[],
    },
};
