use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2363579130: FileFormat = FileFormat {
    id: 2_363_579_130,
    source_type: SourceType::Iana,
    name: "vnd.dwg",
    extensions: &[],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
