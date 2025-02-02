use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4431472809167363624: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mobius plc",
    extensions: &["plc"],
    media_types: &["application/vnd.mobius.plc"],
    internal_signatures: &[],
    related_formats: &[],
};
