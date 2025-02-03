use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3841805673: FileFormat = FileFormat {
    id: 3_841_805_673,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.5gsv2x-local-service-information",
    extensions: &[],
    media_types: &["application/vnd.3gpp.5gsv2x-local-service-information"],
    internal_signatures: &[],
    related_formats: &[],
};
