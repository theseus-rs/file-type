use crate::format::FileFormat;

pub(crate) const LINGUIST_295: FileFormat = FileFormat {
    id: 295,
    puid: "linguist/295",
    name: "Prolog",
    extensions: &["pl", "plt", "pro", "prolog", "yap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
