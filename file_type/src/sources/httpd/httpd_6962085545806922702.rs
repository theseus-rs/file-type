use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6962085545806922702: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "oasis opendocument chart",
    extensions: &["odc"],
    media_types: &["application/vnd.oasis.opendocument.chart"],
    internal_signatures: &[],
    related_formats: &[],
};
