use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1849379916: FileFormat = FileFormat {
    id: 1_849_379_916,
    source_type: SourceType::Iana,
    name: "x-www-form-urlencoded",
    extensions: &[],
    media_types: &["application/x-www-form-urlencoded"],
    internal_signatures: &[],
    related_formats: &[],
};
