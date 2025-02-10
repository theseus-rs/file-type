use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111182275: FileType = FileType {
    file_format: &FileFormat {
        id: 111_182_275,
        source_type: SourceType::Wikidata,
        name: "ActionScript Remote File",
        extensions: &["asr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
