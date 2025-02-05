use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1396309803: FileFormat = FileFormat {
    id: 1_396_309_803,
    source_type: SourceType::Iana,
    name: "vnd.hhe.lesson-player",
    extensions: &[],
    media_types: &["application/vnd.hhe.lesson-player"],
    signatures: &[],
    related_formats: &[],
};
