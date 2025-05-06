use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28049507: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_507,
        source_type: SourceType::Wikidata,
        name: "NEOchrome",
        extensions: &["neo"],
        media_types: &["image/x-neo", "image/x-neochrome"],
        signatures: &[],
        related_formats: &[],
    },
};
