use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_980062566: FileFormat = FileFormat {
    id: 980_062_566,
    source_type: SourceType::Linguist,
    name: "OASv3-json",
    extensions: &["json"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
