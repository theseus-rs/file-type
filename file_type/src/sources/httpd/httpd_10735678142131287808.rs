use crate::format::FileFormat;

pub(crate) const HTTPD_10735678142131287808: FileFormat = FileFormat {
    id: 10_735_678_142_131_287_808,
    puid: "httpd/10735678142131287808",
    name: "pgp signature",
    extensions: &["asc", "sig"],
    media_types: &["application/pgp-signature"],
    internal_signatures: &[],
    related_formats: &[],
};
