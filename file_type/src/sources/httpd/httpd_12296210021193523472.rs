use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12296210021193523472: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rpki ghostbusters",
    extensions: &["gbr"],
    media_types: &["application/rpki-ghostbusters"],
    internal_signatures: &[],
    related_formats: &[],
};
