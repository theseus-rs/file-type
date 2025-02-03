use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5827300586678435774: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mophun certificate",
    extensions: &["mpc"],
    media_types: &["application/vnd.mophun.certificate"],
    internal_signatures: &[],
    related_formats: &[],
};
