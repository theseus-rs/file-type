use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14552983977238544941: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "richtext",
    extensions: &["rtx"],
    media_types: &["text/richtext"],
    internal_signatures: &[],
    related_formats: &[],
};
