use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2483291978: FileFormat = FileFormat {
    id: 2_483_291_978,
    source_type: SourceType::Iana,
    name: "vnd.planar",
    extensions: &[],
    media_types: &["video/vnd.planar"],
    signatures: &[],
    related_formats: &[],
};
