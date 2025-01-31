use crate::format::FileFormat;

pub(crate) const LINGUIST_298: FileFormat = FileFormat {
    id: 298,
    puid: "linguist/298",
    name: "Public Key",
    extensions: &["asc", "pub"],
    media_types: &["application/pgp"],
    internal_signatures: &[],
    related_formats: &[],
};
