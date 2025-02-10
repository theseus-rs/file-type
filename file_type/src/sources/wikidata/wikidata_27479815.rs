use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27479815: FileType = FileType {
    file_format: &FileFormat {
        id: 27_479_815,
        source_type: SourceType::Wikidata,
        name: "7z, version 0.2 (with compression methods version 3.08 beta)",
        extensions: &["7z"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
