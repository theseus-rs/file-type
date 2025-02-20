use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119443865: FileType = FileType {
    file_format: &FileFormat {
        id: 119_443_865,
        source_type: SourceType::Wikidata,
        name: "AutoRoute GB File",
        extensions: &["axg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
