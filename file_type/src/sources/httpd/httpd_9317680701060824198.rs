use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9317680701060824198: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "vtu",
    extensions: &["vtu"],
    media_types: &["model/vnd.vtu"],
    internal_signatures: &[],
    related_formats: &[],
};
