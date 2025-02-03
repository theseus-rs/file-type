use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1049248689: FileFormat = FileFormat {
    id: 1_049_248_689,
    source_type: SourceType::Iana,
    name: "vnd.dece.hd",
    extensions: &[],
    media_types: &["video/vnd.dece.hd"],
    internal_signatures: &[],
    related_formats: &[],
};
