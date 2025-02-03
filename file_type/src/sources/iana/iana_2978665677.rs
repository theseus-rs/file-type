use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2978665677: FileFormat = FileFormat {
    id: 2_978_665_677,
    source_type: SourceType::Iana,
    name: "vnd.claymore",
    extensions: &[],
    media_types: &["application/vnd.claymore"],
    internal_signatures: &[],
    related_formats: &[],
};
