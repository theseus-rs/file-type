use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28858039: FileType = FileType {
    file_format: &FileFormat {
        id: 28_858_039,
        source_type: SourceType::Wikidata,
        name: "Word Binary File Format, version nFibNew=0x00D9",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
