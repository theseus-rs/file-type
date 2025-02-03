use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10132205750207181206: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "simtech mindmapper",
    extensions: &["twd", "twds"],
    media_types: &["application/vnd.simtech-mindmapper"],
    internal_signatures: &[],
    related_formats: &[],
};
