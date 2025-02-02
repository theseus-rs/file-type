use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_641580358: FileFormat = FileFormat {
    id: 641_580_358,
    source_type: SourceType::Linguist,
    name: "Bluespec BH",
    extensions: &["bs"],
    media_types: &["text/x-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
