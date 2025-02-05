use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_228: FileFormat = FileFormat {
    id: 228,
    source_type: SourceType::Linguist,
    name: "Wikitext",
    extensions: &["mediawiki", "wiki", "wikitext"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
