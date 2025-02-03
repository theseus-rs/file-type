use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2205828326: FileFormat = FileFormat {
    id: 2_205_828_326,
    source_type: SourceType::Iana,
    name: "vnd.jsk.isdn-ngn",
    extensions: &[],
    media_types: &["application/vnd.jsk.isdn-ngn"],
    internal_signatures: &[],
    related_formats: &[],
};
