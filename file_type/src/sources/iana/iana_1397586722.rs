use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1397586722: FileFormat = FileFormat {
    id: 1_397_586_722,
    source_type: SourceType::Iana,
    name: "vnd.frogans.fnc (OBSOLETE)",
    extensions: &[],
    media_types: &["application/vnd.frogans.fnc"],
    internal_signatures: &[],
    related_formats: &[],
};
