use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27492801: FileType = FileType {
    file_format: &FileFormat {
        id: 27_492_801,
        source_type: SourceType::Wikidata,
        name: "7z, version 0.2 (with compression methods version 15.06)",
        extensions: &["7z"],
        media_types: &["application/x-7z-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
