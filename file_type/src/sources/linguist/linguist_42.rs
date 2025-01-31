use crate::format::FileFormat;

pub(crate) const LINGUIST_42: FileFormat = FileFormat {
    id: 42,
    puid: "linguist/42",
    name: "C#",
    extensions: &["cake", "cs", "cs.pp", "csx", "linq"],
    media_types: &["text/x-csharp"],
    internal_signatures: &[],
    related_formats: &[],
};
