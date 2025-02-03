use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1399920820: FileFormat = FileFormat {
    id: 1_399_920_820,
    source_type: SourceType::Iana,
    name: "prs.vcfbzip2",
    extensions: &[],
    media_types: &["application/prs.vcfbzip2"],
    internal_signatures: &[],
    related_formats: &[],
};
