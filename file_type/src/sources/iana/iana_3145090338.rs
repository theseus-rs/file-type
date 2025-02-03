use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3145090338: FileFormat = FileFormat {
    id: 3_145_090_338,
    source_type: SourceType::Iana,
    name: "vnd.cups-raw",
    extensions: &[],
    media_types: &["application/vnd.cups-raw"],
    internal_signatures: &[],
    related_formats: &[],
};
