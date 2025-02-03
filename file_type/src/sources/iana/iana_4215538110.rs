use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4215538110: FileFormat = FileFormat {
    id: 4_215_538_110,
    source_type: SourceType::Iana,
    name: "fastinfoset",
    extensions: &[],
    media_types: &["application/fastinfoset"],
    internal_signatures: &[],
    related_formats: &[],
};
