use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27478595: FileType = FileType {
    file_format: &FileFormat {
        id: 27_478_595,
        source_type: SourceType::Wikidata,
        name: "7z, version 0.2 (with compression methods version 4.27, distributed with 7zip v4.27)",
        extensions: &["7z"],
        media_types: &["application/x-7z-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
