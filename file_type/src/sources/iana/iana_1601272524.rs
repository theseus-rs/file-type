use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1601272524: FileFormat = FileFormat {
    id: 1_601_272_524,
    source_type: SourceType::Iana,
    name: "vnd.sbm.mid2",
    extensions: &[],
    media_types: &["application/vnd.sbm.mid2"],
    signatures: &[],
    related_formats: &[],
};
