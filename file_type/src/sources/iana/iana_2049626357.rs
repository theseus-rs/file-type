use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2049626357: FileFormat = FileFormat {
    id: 2_049_626_357,
    source_type: SourceType::Iana,
    name: "vnd.shp",
    extensions: &[],
    media_types: &["application/vnd.shp"],
    internal_signatures: &[],
    related_formats: &[],
};
