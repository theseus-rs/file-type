use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2021704365: FileFormat = FileFormat {
    id: 2_021_704_365,
    source_type: SourceType::Httpd,
    name: "smv",
    extensions: &["smv"],
    media_types: &["video/x-smv"],
    internal_signatures: &[],
    related_formats: &[],
};
