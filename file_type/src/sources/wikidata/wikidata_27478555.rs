use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27478555: FileType = FileType {
    file_format: &FileFormat {
        id: 27_478_555,
        source_type: SourceType::Wikidata,
        name: "7z, version 0.2 (with compression methods version 4.16 beta, distributed with 7zip v4.20)",
        extensions: &["7z"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
