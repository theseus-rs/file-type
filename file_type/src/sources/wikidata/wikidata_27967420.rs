use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967420: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_420,
        source_type: SourceType::Wikidata,
        name: "ANSI Music",
        extensions: &["ams", "mus"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
