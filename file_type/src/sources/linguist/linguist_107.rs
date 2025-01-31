use crate::format::FileFormat;

pub(crate) const LINGUIST_107: FileFormat = FileFormat {
    id: 107,
    puid: "linguist/107",
    name: "Fortran",
    extensions: &["f", "f77", "for", "fpp"],
    media_types: &["text/x-fortran"],
    internal_signatures: &[],
    related_formats: &[],
};
