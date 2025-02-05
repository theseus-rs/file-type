use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3523163259: FileFormat = FileFormat {
    id: 3_523_163_259,
    source_type: SourceType::Iana,
    name: "vnd.insors.igm",
    extensions: &[],
    media_types: &["application/vnd.insors.igm"],
    signatures: &[],
    related_formats: &[],
};
