use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2969229277: FileFormat = FileFormat {
    id: 2_969_229_277,
    source_type: SourceType::Iana,
    name: "jf2feed+json",
    extensions: &[],
    media_types: &["application/jf2feed+json"],
    signatures: &[],
    related_formats: &[],
};
