use crate::format::FileFormat;

pub(crate) const LINGUIST_151: FileFormat = FileFormat {
    id: 151,
    puid: "linguist/151",
    name: "HTML+PHP",
    extensions: &["phtml"],
    media_types: &["application/x-httpd-php"],
    internal_signatures: &[],
    related_formats: &[],
};
