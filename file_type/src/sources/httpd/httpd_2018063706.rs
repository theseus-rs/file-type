use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2018063706: FileFormat = FileFormat {
    id: 2_018_063_706,
    source_type: SourceType::Httpd,
    name: "stardivision calc",
    extensions: &["sdc"],
    media_types: &["application/vnd.stardivision.calc"],
    internal_signatures: &[],
    related_formats: &[],
};
