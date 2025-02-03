use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2925459675: FileFormat = FileFormat {
    id: 2_925_459_675,
    source_type: SourceType::Iana,
    name: "pkix-attr-cert",
    extensions: &[],
    media_types: &["application/pkix-attr-cert"],
    internal_signatures: &[],
    related_formats: &[],
};
