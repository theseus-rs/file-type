use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2803882134: FileFormat = FileFormat {
    id: 2_803_882_134,
    source_type: SourceType::Iana,
    name: "vnd.hbci",
    extensions: &[],
    media_types: &["application/vnd.hbci"],
    signatures: &[],
    related_formats: &[],
};
