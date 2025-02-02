use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_55: FileFormat = FileFormat {
    id: 55,
    source_type: SourceType::Linguist,
    name: "Chapel",
    extensions: &["chpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
