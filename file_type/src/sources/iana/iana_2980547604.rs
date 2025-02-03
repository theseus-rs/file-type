use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2980547604: FileFormat = FileFormat {
    id: 2_980_547_604,
    source_type: SourceType::Iana,
    name: "vnd.uplanet.alert",
    extensions: &[],
    media_types: &["application/vnd.uplanet.alert"],
    internal_signatures: &[],
    related_formats: &[],
};
