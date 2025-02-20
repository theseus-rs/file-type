use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48223065: FileType = FileType {
    file_format: &FileFormat {
        id: 48_223_065,
        source_type: SourceType::Wikidata,
        name: "Turbo Debugger Keystroke Recording File",
        extensions: &["tdk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
