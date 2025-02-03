use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2731887612: FileFormat = FileFormat {
    id: 2_731_887_612,
    source_type: SourceType::Iana,
    name: "pkcs8-encrypted",
    extensions: &[],
    media_types: &["application/pkcs8-encrypted"],
    internal_signatures: &[],
    related_formats: &[],
};
