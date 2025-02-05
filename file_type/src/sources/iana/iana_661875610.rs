use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_661875610: FileFormat = FileFormat {
    id: 661_875_610,
    source_type: SourceType::Iana,
    name: "vnd.belightsoft.lhzl+zip",
    extensions: &[],
    media_types: &["application/vnd.belightsoft.lhzl+zip"],
    signatures: &[],
    related_formats: &[],
};
