use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_170630255: FileFormat = FileFormat {
    id: 170_630_255,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.pic-bw-var",
    extensions: &[],
    media_types: &["application/vnd.3gpp.pic-bw-var"],
    internal_signatures: &[],
    related_formats: &[],
};
