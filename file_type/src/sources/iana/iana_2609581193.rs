use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2609581193: FileFormat = FileFormat {
    id: 2_609_581_193,
    source_type: SourceType::Iana,
    name: "vnd.esmertec.theme-descriptor",
    extensions: &[],
    media_types: &["text/vnd.esmertec.theme-descriptor"],
    signatures: &[],
    related_formats: &[],
};
