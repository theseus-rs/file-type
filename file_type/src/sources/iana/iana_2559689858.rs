use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2559689858: FileFormat = FileFormat {
    id: 2_559_689_858,
    source_type: SourceType::Iana,
    name: "vnd.fujitsu.oasysgp",
    extensions: &[],
    media_types: &["application/vnd.fujitsu.oasysgp"],
    internal_signatures: &[],
    related_formats: &[],
};
