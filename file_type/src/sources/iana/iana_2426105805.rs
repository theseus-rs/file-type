use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2426105805: FileFormat = FileFormat {
    id: 2_426_105_805,
    source_type: SourceType::Iana,
    name: "vnd.dvb.ipdcesgpdd",
    extensions: &[],
    media_types: &["application/vnd.dvb.ipdcesgpdd"],
    signatures: &[],
    related_formats: &[],
};
