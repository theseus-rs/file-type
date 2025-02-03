use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3908223514: FileFormat = FileFormat {
    id: 3_908_223_514,
    source_type: SourceType::Iana,
    name: "vnd.sealed.png",
    extensions: &[],
    media_types: &["image/vnd.sealed.png"],
    internal_signatures: &[],
    related_formats: &[],
};
