use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3530913644: FileFormat = FileFormat {
    id: 3_530_913_644,
    source_type: SourceType::Iana,
    name: "vnd.vcf",
    extensions: &[],
    media_types: &["text/vnd.vcf"],
    signatures: &[],
    related_formats: &[],
};
