use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4781651560126321324: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mseq",
    extensions: &["mseq"],
    media_types: &["application/vnd.mseq"],
    internal_signatures: &[],
    related_formats: &[],
};
