use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27492283: FileType = FileType {
    file_format: &FileFormat {
        id: 27_492_283,
        source_type: SourceType::Wikidata,
        name: "7z, version 0.2 (with compression methods version 9.18)",
        extensions: &["7z"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
