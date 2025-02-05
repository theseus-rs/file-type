use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_421026389: FileFormat = FileFormat {
    id: 421_026_389,
    source_type: SourceType::Linguist,
    name: "CoNLL-U",
    extensions: &["conll", "conllu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
