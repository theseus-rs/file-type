use crate::format::FileFormat;

pub(crate) const LINGUIST_272: FileFormat = FileFormat {
    id: 272,
    puid: "linguist/272",
    name: "PHP",
    extensions: &[
        "aw", "ctp", "fcgi", "inc", "php", "php3", "php4", "php5", "phps", "phpt",
    ],
    media_types: &["application/x-httpd-php"],
    internal_signatures: &[],
    related_formats: &[],
};
