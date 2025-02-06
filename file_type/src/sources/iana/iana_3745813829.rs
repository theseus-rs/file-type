use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3745813829: FileFormat = FileFormat {
    id: 3_745_813_829,
    source_type: SourceType::Iana,
    name: "alto-endpointpropparams+json",
    extensions: &[],
    media_types: &["application/alto-endpointpropparams+json"],
    signatures: &[],
    related_formats: &[],
};
