use crate::format::FileFormat;

pub(crate) const LINGUIST_153: FileFormat = FileFormat {
    id: 153,
    puid: "linguist/153",
    name: "Hack",
    extensions: &["hack", "hh", "hhi", "php"],
    media_types: &["application/x-httpd-php"],
    internal_signatures: &[],
    related_formats: &[],
};
