use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_338: FileFormat = FileFormat {
    id: 338,
    source_type: SourceType::Linguist,
    name: "Sage",
    extensions: &["sage", "sagews"],
    media_types: &["text/x-python"],
    signatures: &[],
    related_formats: &[],
};
