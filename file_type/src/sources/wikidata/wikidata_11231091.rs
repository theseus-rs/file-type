use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_11231091: FileType = FileType {
    file_format: &FileFormat {
        id: 11_231_091,
        source_type: SourceType::Wikidata,
        name: "MASL",
        extensions: &["Msl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
