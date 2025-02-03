use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11775578050658360080: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "texinfo",
    extensions: &["texinfo", "texi"],
    media_types: &["application/x-texinfo"],
    internal_signatures: &[],
    related_formats: &[],
};
