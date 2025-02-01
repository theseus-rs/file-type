use crate::format::FileFormat;

pub(crate) const LINGUIST_254: FileFormat = FileFormat {
    id: 254,
    puid: "linguist/254",
    name: "NumPy",
    extensions: &["numpy", "numpyw", "numsc"],
    media_types: &["text/x-python"],
    internal_signatures: &[],
    related_formats: &[],
};
