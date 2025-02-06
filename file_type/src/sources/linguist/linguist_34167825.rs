use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_34167825: FileFormat = FileFormat {
    id: 34_167_825,
    source_type: SourceType::Linguist,
    name: "Macaulay2",
    extensions: &["m2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
