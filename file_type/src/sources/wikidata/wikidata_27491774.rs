use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27491774: FileType = FileType {
    file_format: &FileFormat {
        id: 27_491_774,
        source_type: SourceType::Wikidata,
        name: "7z, version 0.2 (with compression methods version 9.04)",
        extensions: &["7z"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
