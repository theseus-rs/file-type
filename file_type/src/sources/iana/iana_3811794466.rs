use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3811794466: FileFormat = FileFormat {
    id: 3_811_794_466,
    source_type: SourceType::Iana,
    name: "vnd.hp-PCL",
    extensions: &[],
    media_types: &["application/vnd.hp-PCL"],
    signatures: &[],
    related_formats: &[],
};
