use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28858037: FileType = FileType {
    file_format: &FileFormat {
        id: 28_858_037,
        source_type: SourceType::Wikidata,
        name: "Word Binary File Format, version nFib=0x00C0",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
