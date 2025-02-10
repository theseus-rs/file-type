use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1397586722: FileType = FileType {
    file_format: &FileFormat {
        id: 1_397_586_722,
        source_type: SourceType::Iana,
        name: "vnd.frogans.fnc (OBSOLETE)",
        extensions: &[],
        media_types: &["application/vnd.frogans.fnc"],
        signatures: &[],
        related_formats: &[],
    },
};
