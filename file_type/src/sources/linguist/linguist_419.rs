use crate::format::FileFormat;

pub(crate) const LINGUIST_419: FileFormat = FileFormat {
    id: 419,
    puid: "linguist/419",
    name: "reStructuredText",
    extensions: &["rest", "rest.txt", "rst", "rst.txt"],
    media_types: &["text/x-rst"],
    internal_signatures: &[],
    related_formats: &[],
};
