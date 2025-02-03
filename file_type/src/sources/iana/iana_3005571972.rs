use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3005571972: FileFormat = FileFormat {
    id: 3_005_571_972,
    source_type: SourceType::Iana,
    name: "vnd.byu.uapi+json",
    extensions: &[],
    media_types: &["application/vnd.byu.uapi+json"],
    internal_signatures: &[],
    related_formats: &[],
};
