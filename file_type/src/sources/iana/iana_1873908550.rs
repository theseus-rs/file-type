use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1873908550: FileFormat = FileFormat {
    id: 1_873_908_550,
    source_type: SourceType::Iana,
    name: "vnd.ims.lti.v2.toolsettings.simple+json",
    extensions: &[],
    media_types: &["application/vnd.ims.lti.v2.toolsettings.simple+json"],
    signatures: &[],
    related_formats: &[],
};
