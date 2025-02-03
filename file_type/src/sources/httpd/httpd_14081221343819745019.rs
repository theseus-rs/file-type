use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14081221343819745019: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cooltalk",
    extensions: &["ice"],
    media_types: &["x-conference/x-cooltalk"],
    internal_signatures: &[],
    related_formats: &[],
};
