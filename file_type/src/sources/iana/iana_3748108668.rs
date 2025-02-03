use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3748108668: FileFormat = FileFormat {
    id: 3_748_108_668,
    source_type: SourceType::Iana,
    name: "vnd.sealedmedia.softseal.pdf",
    extensions: &[],
    media_types: &["application/vnd.sealedmedia.softseal.pdf"],
    internal_signatures: &[],
    related_formats: &[],
};
