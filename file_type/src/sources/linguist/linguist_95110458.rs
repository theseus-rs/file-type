use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_95110458: FileFormat = FileFormat {
    id: 95_110_458,
    source_type: SourceType::Linguist,
    name: "Glimmer TS",
    extensions: &["gts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
