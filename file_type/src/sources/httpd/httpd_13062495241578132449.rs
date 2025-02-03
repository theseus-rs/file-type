use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13062495241578132449: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "setext",
    extensions: &["etx"],
    media_types: &["text/x-setext"],
    internal_signatures: &[],
    related_formats: &[],
};
