use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27479976: FileType = FileType {
    file_format: &FileFormat {
        id: 27_479_976,
        source_type: SourceType::Wikidata,
        name: "7z, version 0.2 (with compression methods version 4.43)",
        extensions: &["7z"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
