use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_776095572: FileFormat = FileFormat {
    id: 776_095_572,
    source_type: SourceType::Iana,
    name: "vc2",
    extensions: &[],
    media_types: &["video/vc2"],
    internal_signatures: &[],
    related_formats: &[],
};
