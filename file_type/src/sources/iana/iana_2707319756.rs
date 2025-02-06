use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2707319756: FileFormat = FileFormat {
    id: 2_707_319_756,
    source_type: SourceType::Iana,
    name: "vnd.wap.wmlscriptc",
    extensions: &[],
    media_types: &["application/vnd.wap.wmlscriptc"],
    signatures: &[],
    related_formats: &[],
};
