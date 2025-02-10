use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967438: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_438,
        source_type: SourceType::Wikidata,
        name: "Digital Picture Exchange, version 2",
        extensions: &["dpx"],
        media_types: &["image/x-dpx"],
        signatures: &[],
        related_formats: &[],
    },
};
