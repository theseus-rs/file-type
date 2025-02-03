use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_159747167: FileFormat = FileFormat {
    id: 159_747_167,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.pic-bw-small",
    extensions: &[],
    media_types: &["application/vnd.3gpp.pic-bw-small"],
    internal_signatures: &[],
    related_formats: &[],
};
