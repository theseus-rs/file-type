use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_565406949: FileFormat = FileFormat {
    id: 565_406_949,
    source_type: SourceType::Iana,
    name: "vnd.intergeo",
    extensions: &[],
    media_types: &["application/vnd.intergeo"],
    internal_signatures: &[],
    related_formats: &[],
};
