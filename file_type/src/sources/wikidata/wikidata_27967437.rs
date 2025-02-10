use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967437: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_437,
        source_type: SourceType::Wikidata,
        name: "Digital Picture Exchange, version 1",
        extensions: &["dpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
