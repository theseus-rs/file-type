use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_248: FileFormat = FileFormat {
    id: 248,
    source_type: SourceType::Linguist,
    name: "Nginx",
    extensions: &["nginx", "nginxconf", "vhost"],
    media_types: &["text/x-nginx-conf"],
    internal_signatures: &[],
    related_formats: &[],
};
