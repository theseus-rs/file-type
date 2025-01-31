use crate::format::FileFormat;

pub(crate) const LINGUIST_89: FileFormat = FileFormat {
    id: 89,
    puid: "linguist/89",
    name: "Dockerfile",
    extensions: &["containerfile", "dockerfile"],
    media_types: &["text/x-dockerfile"],
    internal_signatures: &[],
    related_formats: &[],
};
