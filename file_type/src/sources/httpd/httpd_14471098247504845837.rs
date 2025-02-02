use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14471098247504845837: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "nokia n gage data",
    extensions: &["ngdat"],
    media_types: &["application/vnd.nokia.n-gage.data"],
    internal_signatures: &[],
    related_formats: &[],
};
