use crate::format::FileFormat;

pub(crate) const LINGUIST_185: FileFormat = FileFormat {
    id: 185,
    puid: "linguist/185",
    name: "Jupyter Notebook",
    extensions: &["ipynb"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
