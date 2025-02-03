use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1451282716: FileFormat = FileFormat {
    id: 1_451_282_716,
    source_type: SourceType::Httpd,
    name: "stardivision writer global",
    extensions: &["sgl"],
    media_types: &["application/vnd.stardivision.writer-global"],
    internal_signatures: &[],
    related_formats: &[],
};
