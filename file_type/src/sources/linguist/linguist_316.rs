use crate::format::FileFormat;

pub(crate) const LINGUIST_316: FileFormat = FileFormat {
    id: 316,
    puid: "linguist/316",
    name: "Racket",
    extensions: &["rkt", "rktd", "rktl", "scrbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
