use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10480217407537137783: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "yamaha openscoreformat osfpvg xml",
    extensions: &["osfpvg"],
    media_types: &["application/vnd.yamaha.openscoreformat.osfpvg+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
