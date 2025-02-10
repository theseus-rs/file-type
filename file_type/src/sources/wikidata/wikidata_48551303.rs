use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48551303: FileType = FileType {
    file_format: &FileFormat {
        id: 48_551_303,
        source_type: SourceType::Wikidata,
        name: "Word Perfect for Windows Document file format",
        extensions: &["w52", "wp", "wp5", "wpd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
