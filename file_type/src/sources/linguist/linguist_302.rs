use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_302: FileFormat = FileFormat {
    id: 302,
    source_type: SourceType::Linguist,
    name: "PureScript",
    extensions: &["purs"],
    media_types: &["text/x-haskell"],
    signatures: &[],
    related_formats: &[],
};
