use crate::format::FileFormat;

pub(crate) const LINGUIST_248: FileFormat = FileFormat {
    id: 248,
    puid: "linguist/248",
    name: "Nginx",
    extensions: &["nginx", "nginxconf", "vhost"],
    media_types: &["text/x-nginx-conf"],
    internal_signatures: &[],
    related_formats: &[],
};
