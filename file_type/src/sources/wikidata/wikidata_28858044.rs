use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28858044: FileType = FileType {
    file_format: &FileFormat {
        id: 28_858_044,
        source_type: SourceType::Wikidata,
        name: "Word Binary File Format, version nFibNew=0x0112",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
