use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_357046146: FileFormat = FileFormat {
    id: 357_046_146,
    source_type: SourceType::Linguist,
    name: "Closure Templates",
    extensions: &["soy"],
    media_types: &["text/x-soy"],
    internal_signatures: &[],
    related_formats: &[],
};
