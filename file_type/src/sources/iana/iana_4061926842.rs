use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4061926842: FileFormat = FileFormat {
    id: 4_061_926_842,
    source_type: SourceType::Iana,
    name: "vnd.dece.zip",
    extensions: &[],
    media_types: &["application/vnd.dece.zip"],
    internal_signatures: &[],
    related_formats: &[],
};
