use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122412480: FileType = FileType {
    file_format: &FileFormat {
        id: 122_412_480,
        source_type: SourceType::Wikidata,
        name: "Merge File",
        extensions: &["mer"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
