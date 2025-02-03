use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2859263887: FileFormat = FileFormat {
    id: 2_859_263_887,
    source_type: SourceType::Httpd,
    name: "lotus approach",
    extensions: &["apr"],
    media_types: &["application/vnd.lotus-approach"],
    internal_signatures: &[],
    related_formats: &[],
};
