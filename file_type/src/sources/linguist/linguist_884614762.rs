use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_884614762: FileFormat = FileFormat {
    id: 884_614_762,
    source_type: SourceType::Linguist,
    name: "Adblock Filter List",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
