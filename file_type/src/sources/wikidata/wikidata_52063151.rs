use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_52063151: FileType = FileType {
    file_format: &FileFormat {
        id: 52_063_151,
        source_type: SourceType::Wikidata,
        name: "Lotus Notes File",
        extensions: &["box"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
