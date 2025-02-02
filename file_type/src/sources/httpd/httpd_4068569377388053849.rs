use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4068569377388053849: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "conference",
    extensions: &["nsc"],
    media_types: &["application/x-conference"],
    internal_signatures: &[],
    related_formats: &[],
};
