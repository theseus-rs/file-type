use crate::format::FileFormat;

pub(crate) const LINGUIST_291: FileFormat = FileFormat {
    id: 291,
    puid: "linguist/291",
    name: "PostScript",
    extensions: &["eps", "epsi", "pfa", "ps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
