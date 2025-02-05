use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_960802919: FileFormat = FileFormat {
    id: 960_802_919,
    source_type: SourceType::Iana,
    name: "vnd.xara",
    extensions: &[],
    media_types: &["application/vnd.xara"],
    signatures: &[],
    related_formats: &[],
};
