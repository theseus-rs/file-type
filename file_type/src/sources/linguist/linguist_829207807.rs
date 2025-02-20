use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_829207807: FileType = FileType {
    file_format: &FileFormat {
        id: 829_207_807,
        source_type: SourceType::Linguist,
        name: "CameLIGO",
        extensions: &["mligo"],
        media_types: &["text/x-ocaml"],
        signatures: &[],
        related_formats: &[],
    },
};
