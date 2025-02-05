use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_374317347: FileFormat = FileFormat {
    id: 374_317_347,
    source_type: SourceType::Linguist,
    name: "OpenType Feature File",
    extensions: &["fea"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
