use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_677095381: FileType = FileType {
    file_format: &FileFormat {
        id: 677_095_381,
        source_type: SourceType::Linguist,
        name: "Cabal Config",
        extensions: &["cabal"],
        media_types: &["text/x-haskell"],
        signatures: &[],
        related_formats: &[],
    },
};
