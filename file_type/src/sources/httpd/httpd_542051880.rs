use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_542051880: FileFormat = FileFormat {
    id: 542_051_880,
    source_type: SourceType::Httpd,
    name: "zmachine",
    extensions: &["z1", "z2", "z3", "z4", "z5", "z6", "z7", "z8"],
    media_types: &["application/x-zmachine"],
    internal_signatures: &[],
    related_formats: &[],
};
