use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2210814693: FileFormat = FileFormat {
    id: 2_210_814_693,
    source_type: SourceType::Iana,
    name: "vnd.aplextor.warrp+json",
    extensions: &[],
    media_types: &["application/vnd.aplextor.warrp+json"],
    signatures: &[],
    related_formats: &[],
};
