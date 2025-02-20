use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_42397505: FileType = FileType {
    file_format: &FileFormat {
        id: 42_397_505,
        source_type: SourceType::Wikidata,
        name: "vimwiki",
        extensions: &["wiki"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
