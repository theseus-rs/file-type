use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105099750: FileType = FileType {
    file_format: &FileFormat {
        id: 105_099_750,
        source_type: SourceType::Wikidata,
        name: ".ipynb",
        extensions: &["ipynb"],
        media_types: &["application/x-ipynb+json"],
        signatures: &[],
        related_formats: &[],
    },
};
