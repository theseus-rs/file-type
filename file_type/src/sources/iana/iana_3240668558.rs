use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3240668558: FileFormat = FileFormat {
    id: 3_240_668_558,
    source_type: SourceType::Iana,
    name: "vnd.spotfire.sfs",
    extensions: &[],
    media_types: &["application/vnd.spotfire.sfs"],
    internal_signatures: &[],
    related_formats: &[],
};
