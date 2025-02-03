use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3353041256: FileFormat = FileFormat {
    id: 3_353_041_256,
    source_type: SourceType::Httpd,
    name: "mobius mbk",
    extensions: &["mbk"],
    media_types: &["application/vnd.mobius.mbk"],
    internal_signatures: &[],
    related_formats: &[],
};
