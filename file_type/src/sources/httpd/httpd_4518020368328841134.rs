use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4518020368328841134: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "prs cww",
    extensions: &["cww"],
    media_types: &["application/prs.cww"],
    internal_signatures: &[],
    related_formats: &[],
};
