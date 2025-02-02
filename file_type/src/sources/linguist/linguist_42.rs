use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_42: FileFormat = FileFormat {
    id: 42,
    source_type: SourceType::Linguist,
    name: "C#",
    extensions: &["cake", "cs", "cs.pp", "csx", "linq"],
    media_types: &["text/x-csharp"],
    internal_signatures: &[],
    related_formats: &[],
};
