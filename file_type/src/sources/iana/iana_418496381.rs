use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_418496381: FileFormat = FileFormat {
    id: 418_496_381,
    source_type: SourceType::Iana,
    name: "vnd.uplanet.signal",
    extensions: &[],
    media_types: &["application/vnd.uplanet.signal"],
    internal_signatures: &[],
    related_formats: &[],
};
