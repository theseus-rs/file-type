use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_891511949: FileFormat = FileFormat {
    id: 891_511_949,
    source_type: SourceType::Iana,
    name: "vnd.ecip.rlp",
    extensions: &[],
    media_types: &["application/vnd.ecip.rlp"],
    internal_signatures: &[],
    related_formats: &[],
};
