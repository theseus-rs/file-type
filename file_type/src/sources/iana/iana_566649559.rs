use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_566649559: FileFormat = FileFormat {
    id: 566_649_559,
    source_type: SourceType::Iana,
    name: "ODX",
    extensions: &[],
    media_types: &["application/ODX"],
    internal_signatures: &[],
    related_formats: &[],
};
