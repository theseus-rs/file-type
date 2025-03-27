use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28858042: FileType = FileType {
    file_format: &FileFormat {
        id: 28_858_042,
        source_type: SourceType::Wikidata,
        name: "Word Binary File Format, version nFibNew=0x010C",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
