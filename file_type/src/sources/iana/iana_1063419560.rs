use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1063419560: FileFormat = FileFormat {
    id: 1_063_419_560,
    source_type: SourceType::Iana,
    name: "vnd.ims.lis.v2.result+json",
    extensions: &[],
    media_types: &["application/vnd.ims.lis.v2.result+json"],
    internal_signatures: &[],
    related_formats: &[],
};
