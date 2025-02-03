use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2712046030815474526: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "epson ssf",
    extensions: &["ssf"],
    media_types: &["application/vnd.epson.ssf"],
    internal_signatures: &[],
    related_formats: &[],
};
