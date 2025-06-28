use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1061635506: FileType = FileType {
    file_format: &FileFormat {
        id: 1_061_635_506,
        source_type: SourceType::Linguist,
        name: "Untyped Plutus Core",
        extensions: &["uplc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
