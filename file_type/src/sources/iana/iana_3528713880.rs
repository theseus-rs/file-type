use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3528713880: FileFormat = FileFormat {
    id: 3_528_713_880,
    source_type: SourceType::Iana,
    name: "voucher-cms+json",
    extensions: &[],
    media_types: &["application/voucher-cms+json"],
    signatures: &[],
    related_formats: &[],
};
