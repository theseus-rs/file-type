use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_93431491: FileType = FileType {
    file_format: &FileFormat {
        id: 93_431_491,
        source_type: SourceType::Wikidata,
        name: "Final Draft Document 8",
        extensions: &["fdx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
