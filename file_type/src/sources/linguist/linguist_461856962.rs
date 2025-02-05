use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_461856962: FileFormat = FileFormat {
    id: 461_856_962,
    source_type: SourceType::Linguist,
    name: "Nunjucks",
    extensions: &["njk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
