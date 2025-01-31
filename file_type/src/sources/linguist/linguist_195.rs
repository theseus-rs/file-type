use crate::format::FileFormat;

pub(crate) const LINGUIST_195: FileFormat = FileFormat {
    id: 195,
    puid: "linguist/195",
    name: "Lasso",
    extensions: &["las", "lasso", "lasso8", "lasso9"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
