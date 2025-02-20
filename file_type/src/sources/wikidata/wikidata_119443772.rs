use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119443772: FileType = FileType {
    file_format: &FileFormat {
        id: 119_443_772,
        source_type: SourceType::Wikidata,
        name: "AutoRoute File",
        extensions: &["axe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
