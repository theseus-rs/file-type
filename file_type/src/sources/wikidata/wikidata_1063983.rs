use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1063983: FileType = FileType {
    file_format: &FileFormat {
        id: 1_063_983,
        source_type: SourceType::Wikidata,
        name: "H.263",
        extensions: &[],
        media_types: &["video/H263-1998", "video/H263-2000"],
        signatures: &[],
        related_formats: &[],
    },
};
