use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1359614180: FileFormat = FileFormat {
    id: 1_359_614_180,
    source_type: SourceType::Iana,
    name: "vnd.gdl",
    extensions: &[],
    media_types: &["model/vnd.gdl"],
    signatures: &[],
    related_formats: &[],
};
