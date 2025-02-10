use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_421026389: FileType = FileType {
    file_format: &FileFormat {
        id: 421_026_389,
        source_type: SourceType::Linguist,
        name: "CoNLL-U",
        extensions: &["conll", "conllu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
