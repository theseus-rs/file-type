use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3089008901: FileFormat = FileFormat {
    id: 3_089_008_901,
    source_type: SourceType::Iana,
    name: "vnd.d2l.coursepackage1p0+zip",
    extensions: &[],
    media_types: &["application/vnd.d2l.coursepackage1p0+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
