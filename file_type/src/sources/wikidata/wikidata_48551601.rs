use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48551601: FileType = FileType {
    file_format: &FileFormat {
        id: 48_551_601,
        source_type: SourceType::Wikidata,
        name: "Microsoft Word for Windows Macro",
        extensions: &["wpm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
