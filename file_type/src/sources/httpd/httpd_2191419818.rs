use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2191419818: FileFormat = FileFormat {
    id: 2_191_419_818,
    source_type: SourceType::Httpd,
    name: "sun xml writer global",
    extensions: &["sxg"],
    media_types: &["application/vnd.sun.xml.writer.global"],
    signatures: &[],
    related_formats: &[],
};
