use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_83443959: FileType = FileType {
    file_format: &FileFormat {
        id: 83_443_959,
        source_type: SourceType::Wikidata,
        name: "Terse Executable",
        extensions: &["efi", "te"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
