use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3366619709: FileFormat = FileFormat {
    id: 3_366_619_709,
    source_type: SourceType::Iana,
    name: "vnd.ims.lti.v2.toolconsumerprofile+json",
    extensions: &[],
    media_types: &["application/vnd.ims.lti.v2.toolconsumerprofile+json"],
    signatures: &[],
    related_formats: &[],
};
