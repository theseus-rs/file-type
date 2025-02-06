use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4210995188: FileFormat = FileFormat {
    id: 4_210_995_188,
    source_type: SourceType::Iana,
    name: "vnd.ims.lti.v2.toolproxy.id+json",
    extensions: &[],
    media_types: &["application/vnd.ims.lti.v2.toolproxy.id+json"],
    signatures: &[],
    related_formats: &[],
};
