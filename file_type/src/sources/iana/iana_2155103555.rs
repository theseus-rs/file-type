use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2155103555: FileFormat = FileFormat {
    id: 2_155_103_555,
    source_type: SourceType::Iana,
    name: "vnd.sbm.cid",
    extensions: &[],
    media_types: &["application/vnd.sbm.cid"],
    signatures: &[],
    related_formats: &[],
};
