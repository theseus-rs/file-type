use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3008497538: FileFormat = FileFormat {
    id: 3_008_497_538,
    source_type: SourceType::Iana,
    name: "vnd.ntt-local.sip-ta_tcp_stream",
    extensions: &[],
    media_types: &["application/vnd.ntt-local.sip-ta_tcp_stream"],
    internal_signatures: &[],
    related_formats: &[],
};
