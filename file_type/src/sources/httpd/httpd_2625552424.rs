use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2625552424: FileFormat = FileFormat {
    id: 2_625_552_424,
    source_type: SourceType::Httpd,
    name: "wolfram player",
    extensions: &["nbp"],
    media_types: &["application/vnd.wolfram.player"],
    signatures: &[],
    related_formats: &[],
};
