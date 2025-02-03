use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_668848572: FileFormat = FileFormat {
    id: 668_848_572,
    source_type: SourceType::Iana,
    name: "vnd.fujitsu.oasys3",
    extensions: &[],
    media_types: &["application/vnd.fujitsu.oasys3"],
    internal_signatures: &[],
    related_formats: &[],
};
