use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110443175: FileType = FileType {
    file_format: &FileFormat {
        id: 110_443_175,
        source_type: SourceType::Wikidata,
        name: "Visual Basics MAK File",
        extensions: &["mak"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
