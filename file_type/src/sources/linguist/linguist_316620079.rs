use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_316620079: FileFormat = FileFormat {
    id: 316_620_079,
    source_type: SourceType::Linguist,
    name: "JCL",
    extensions: &["jcl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
