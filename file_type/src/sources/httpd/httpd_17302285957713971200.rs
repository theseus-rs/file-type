use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17302285957713971200: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mobius mbk",
    extensions: &["mbk"],
    media_types: &["application/vnd.mobius.mbk"],
    internal_signatures: &[],
    related_formats: &[],
};
