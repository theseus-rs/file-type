use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100596765: FileType = FileType {
    file_format: &FileFormat {
        id: 100_596_765,
        source_type: SourceType::Wikidata,
        name: "Minitab Project, version 12-13",
        extensions: &["mpj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
