use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2196028526: FileFormat = FileFormat {
    id: 2_196_028_526,
    source_type: SourceType::Iana,
    name: "prs.hpub+zip",
    extensions: &[],
    media_types: &["application/prs.hpub+zip"],
    signatures: &[],
    related_formats: &[],
};
