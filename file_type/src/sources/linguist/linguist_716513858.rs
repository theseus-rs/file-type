use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_716513858: FileFormat = FileFormat {
    id: 716_513_858,
    source_type: SourceType::Linguist,
    name: "Proguard",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
