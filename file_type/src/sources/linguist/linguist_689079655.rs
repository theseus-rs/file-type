use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_689079655: FileFormat = FileFormat {
    id: 689_079_655,
    source_type: SourceType::Linguist,
    name: "OverpassQL",
    extensions: &["overpassql"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
