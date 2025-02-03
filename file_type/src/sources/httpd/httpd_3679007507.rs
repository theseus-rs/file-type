use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3679007507: FileFormat = FileFormat {
    id: 3_679_007_507,
    source_type: SourceType::Httpd,
    name: "conference",
    extensions: &["nsc"],
    media_types: &["application/x-conference"],
    internal_signatures: &[],
    related_formats: &[],
};
