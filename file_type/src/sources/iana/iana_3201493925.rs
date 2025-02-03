use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3201493925: FileFormat = FileFormat {
    id: 3_201_493_925,
    source_type: SourceType::Iana,
    name: "pgp-encrypted",
    extensions: &[],
    media_types: &["application/pgp-encrypted"],
    internal_signatures: &[],
    related_formats: &[],
};
