use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2383649697: FileFormat = FileFormat {
    id: 2_383_649_697,
    source_type: SourceType::Httpd,
    name: "tcl",
    extensions: &["tcl"],
    media_types: &["application/x-tcl"],
    internal_signatures: &[],
    related_formats: &[],
};
