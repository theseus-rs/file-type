use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3340204496: FileFormat = FileFormat {
    id: 3_340_204_496,
    source_type: SourceType::Iana,
    name: "slate",
    extensions: &[],
    media_types: &["application/slate"],
    internal_signatures: &[],
    related_formats: &[],
};
