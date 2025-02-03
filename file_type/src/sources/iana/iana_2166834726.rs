use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2166834726: FileFormat = FileFormat {
    id: 2_166_834_726,
    source_type: SourceType::Iana,
    name: "sip",
    extensions: &[],
    media_types: &["message/sip"],
    internal_signatures: &[],
    related_formats: &[],
};
