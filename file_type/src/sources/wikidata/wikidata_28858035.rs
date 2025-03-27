use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28858035: FileType = FileType {
    file_format: &FileFormat {
        id: 28_858_035,
        source_type: SourceType::Wikidata,
        name: "Word Binary File Format, version nFib=0x00C1",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
