use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4847972083146275462: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "oasis opendocument formula",
    extensions: &["odf"],
    media_types: &["application/vnd.oasis.opendocument.formula"],
    internal_signatures: &[],
    related_formats: &[],
};
