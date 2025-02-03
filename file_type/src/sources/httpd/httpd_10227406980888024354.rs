use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10227406980888024354: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "yamaha openscoreformat",
    extensions: &["osf"],
    media_types: &["application/vnd.yamaha.openscoreformat"],
    internal_signatures: &[],
    related_formats: &[],
};
