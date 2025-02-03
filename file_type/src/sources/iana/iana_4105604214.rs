use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4105604214: FileFormat = FileFormat {
    id: 4_105_604_214,
    source_type: SourceType::Iana,
    name: "vnd.veraison.tsm-report+cbor",
    extensions: &[],
    media_types: &["application/vnd.veraison.tsm-report+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
